"""User service for business logic operations."""

from typing import Optional, List, Dict, Any
from app.extensions import db
from app.models.user import User
from app.utils.validators import validate_email, validate_password
from app.utils.exceptions import ValidationError, NotFoundError, ConflictError
import logging

logger = logging.getLogger(__name__)


class UserService:
    """Service class for user-related operations."""
    
    @staticmethod
    def create_user(username: str, email: str, password: str, **kwargs) -> User:
        """Create a new user with validation.
        
        Args:
            username: Unique username
            email: User's email address
            password: User's password (will be hashed)
            
        Returns:
            Created User instance
            
        Raises:
            ValidationError: If validation fails
            ConflictError: If username or email already exists
        """
        # Validate input
        if not username or len(username) < 3:
            raise ValidationError("Username must be at least 3 characters")
        
        if not validate_email(email):
            raise ValidationError("Invalid email address")
        
        if not validate_password(password):
            raise ValidationError(
                "Password must be at least 8 characters and contain "
                "uppercase, lowercase, and numeric characters"
            )
        
        # Check for existing user
        if User.query.filter_by(username=username).first():
            raise ConflictError("Username already exists")
        
        if User.query.filter_by(email=email).first():
            raise ConflictError("Email already registered")
        
        # Create user
        user = User(username=username, email=email, **kwargs)
        user.set_password(password)
        
        try:
            db.session.add(user)
            db.session.commit()
            logger.info(f"Created new user: {username}")
            return user
        except Exception as e:
            db.session.rollback()
            logger.error(f"Failed to create user: {e}")
            raise
    
    @staticmethod
    def get_user_by_id(user_id: int) -> Optional[User]:
        """Retrieve a user by ID.
        
        Args:
            user_id: The user's ID
            
        Returns:
            User instance or None if not found
        """
        return User.query.get(user_id)
    
    @staticmethod
    def get_user_by_username(username: str) -> Optional[User]:
        """Retrieve a user by username.
        
        Args:
            username: The user's username
            
        Returns:
            User instance or None if not found
        """
        return User.query.filter_by(username=username).first()
    
    @staticmethod
    def get_all_users(page: int = 1, per_page: int = 20, 
                      include_inactive: bool = False) -> Dict[str, Any]:
        """Get paginated list of users.
        
        Args:
            page: Page number (1-indexed)
            per_page: Number of users per page
            include_inactive: Whether to include inactive users
            
        Returns:
            Dictionary with users and pagination info
        """
        query = User.query
        
        if not include_inactive:
            query = query.filter_by(is_active=True)
        
        pagination = query.order_by(User.created_at.desc()).paginate(
            page=page, per_page=per_page, error_out=False
        )
        
        return {
            'users': [user.to_dict(include_email=True) for user in pagination.items],
            'total': pagination.total,
            'page': page,
            'per_page': per_page,
            'pages': pagination.pages
        }
    
    @staticmethod
    def update_user(user_id: int, **kwargs) -> User:
        """Update user information.
        
        Args:
            user_id: The user's ID
            **kwargs: Fields to update
            
        Returns:
            Updated User instance
            
        Raises:
            NotFoundError: If user not found
        """
        user = User.query.get(user_id)
        if not user:
            raise NotFoundError("User not found")
        
        # Handle email update
        if 'email' in kwargs:
            new_email = kwargs.pop('email')
            if new_email != user.email:
                if User.query.filter_by(email=new_email).first():
                    raise ConflictError("Email already in use")
                user.email = new_email
        
        # Handle password update
        if 'password' in kwargs:
            if not validate_password(kwargs['password']):
                raise ValidationError("Invalid password")
            user.set_password(kwargs.pop('password'))
        
        # Update other fields
        for key, value in kwargs.items():
            if hasattr(user, key):
                setattr(user, key, value)
        
        try:
            db.session.commit()
            logger.info(f"Updated user {user_id}")
            return user
        except Exception as e:
            db.session.rollback()
            logger.error(f"Failed to update user: {e}")
            raise
    
    @staticmethod
    def delete_user(user_id: int, soft: bool = True) -> bool:
        """Delete a user.
        
        Args:
            user_id: The user's ID
            soft: If True, deactivate user; if False, permanently delete
            
        Returns:
            True if successful
            
        Raises:
            NotFoundError: If user not found
        """
        user = User.query.get(user_id)
        if not user:
            raise NotFoundError("User not found")
        
        if soft:
            user.is_active = False
            db.session.commit()
            logger.info(f"Deactivated user {user_id}")
        else:
            db.session.delete(user)
            db.session.commit()
            logger.info(f"Permanently deleted user {user_id}")
        
        return True
    
    @staticmethod
    def search_users(query: str, limit: int = 10) -> List[User]:
        """Search users by username or email.
        
        Args:
            query: Search query string
            limit: Maximum number of results
            
        Returns:
            List of matching User instances
        """
        search_pattern = f"%{query}%"
        return User.query.filter(
            db.or_(
                User.username.ilike(search_pattern),
                User.email.ilike(search_pattern)
            )
        ).limit(limit).all()
