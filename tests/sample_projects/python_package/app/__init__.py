"""Main application module for the Python REST API."""

from flask import Flask, jsonify, request
from app.routes import register_routes
from app.extensions import db, migrate, jwt
from config import config
import os

def create_app(config_name=None):
    """Application factory pattern for creating Flask app."""
    if config_name is None:
        config_name = os.getenv('FLASK_ENV', 'development')
    
    app = Flask(__name__)
    app.config.from_object(config[config_name])
    
    # Initialize extensions
    db.init_app(app)
    migrate.init_app(app, db)
    jwt.init_app(app)
    
    # Register blueprints
    register_routes(app)
    
    # Register error handlers
    register_error_handlers(app)
    
    @app.route('/health')
    def health_check():
        return jsonify({'status': 'healthy', 'environment': config_name})
    
    return app

def register_error_handlers(app):
    """Register custom error handlers."""
    from werkzeug.exceptions import HTTPException
    
    @app.errorhandler(HTTPException)
    def handle_http_exception(e):
        return jsonify({
            'error': e.name,
            'message': e.description,
            'status': e.code
        }), e.code
    
    @app.errorhandler(404)
    def handle_not_found(e):
        return jsonify({
            'error': 'Not Found',
            'message': 'The requested resource was not found',
            'status': 404
        }), 404
    
    @app.errorhandler(500)
    def handle_server_error(e):
        return jsonify({
            'error': 'Internal Server Error',
            'message': 'An unexpected error occurred',
            'status': 500
        }), 500

if __name__ == '__main__':
    app = create_app()
    app.run(host='0.0.0.0', port=5000, debug=True)
