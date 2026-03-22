"""API routes for user resources."""

from flask import Blueprint, request, jsonify
from app.services.user_service import UserService
from app.utils.decorators import require_auth, require_admin
from app.utils.exceptions import ValidationError, NotFoundError, ConflictError
from app.schemas.user_schema import UserSchema
from marshmallow import ValidationError as MarshmallowValidationError

users_bp = Blueprint('users', __name__, url_prefix='/api/v1/users')
user_schema = UserSchema()


@users_bp.route('', methods=['GET'])
@require_auth
def get_users():
    """Get paginated list of users."""
    page = request.args.get('page', 1, type=int)
    per_page = request.args.get('per_page', 20, type=int)
    include_inactive = request.args.get('include_inactive', 'false').lower() == 'true'
    
    result = UserService.get_all_users(
        page=page, 
        per_page=min(per_page, 100),
        include_inactive=include_inactive
    )
    
    return jsonify(result), 200


@users_bp.route('/<int:user_id>', methods=['GET'])
@require_auth
def get_user(user_id):
    """Get a specific user by ID."""
    user = UserService.get_user_by_id(user_id)
    if not user:
        raise NotFoundError("User not found")
    
    return jsonify(user.to_dict(include_email=True)), 200


@users_bp.route('', methods=['POST'])
def create_user():
    """Create a new user."""
    try:
        data = user_schema.load(request.json)
    except MarshmallowValidationError as e:
        return jsonify({'errors': e.messages}), 400
    
    try:
        user = UserService.create_user(**data)
        return jsonify(user.to_dict(include_email=True)), 201
    except ValidationError as e:
        return jsonify({'error': str(e)}), 400
    except ConflictError as e:
        return jsonify({'error': str(e)}), 409


@users_bp.route('/<int:user_id>', methods=['PUT'])
@require_auth
def update_user(user_id):
    """Update an existing user."""
    try:
        data = user_schema.load(request.json, partial=True)
    except MarshmallowValidationError as e:
        return jsonify({'errors': e.messages}), 400
    
    try:
        user = UserService.update_user(user_id, **data)
        return jsonify(user.to_dict(include_email=True)), 200
    except NotFoundError as e:
        return jsonify({'error': str(e)}), 404
    except ValidationError as e:
        return jsonify({'error': str(e)}), 400
    except ConflictError as e:
        return jsonify({'error': str(e)}), 409


@users_bp.route('/<int:user_id>', methods=['DELETE'])
@require_auth
@require_admin
def delete_user(user_id):
    """Delete a user (admin only)."""
    try:
        UserService.delete_user(user_id)
        return '', 204
    except NotFoundError as e:
        return jsonify({'error': str(e)}), 404


@users_bp.route('/search', methods=['GET'])
@require_auth
def search_users():
    """Search users by username or email."""
    query = request.args.get('q', '')
    if len(query) < 2:
        return jsonify({'error': 'Query must be at least 2 characters'}), 400
    
    limit = request.args.get('limit', 10, type=int)
    users = UserService.search_users(query, limit=min(limit, 50))
    
    return jsonify({
        'query': query,
        'results': [user.to_dict() for user in users],
        'count': len(users)
    }), 200
