import { useState, useEffect, useCallback } from 'react';
import type { User, AuthState, LoginCredentials, RegisterData } from '../types';
import { authApi } from '../api/client';
import { useLocalStorage } from './useLocalStorage';
import { useToast } from './useToast';

interface UseAuthReturn {
  user: User | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  login: (credentials: LoginCredentials) => Promise<void>;
  register: (data: RegisterData) => Promise<void>;
  logout: () => void;
  updateProfile: (data: Partial<User>) => Promise<void>;
  resetPassword: (email: string) => Promise<void>;
}

export function useAuth(): UseAuthReturn {
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [token, setToken] = useLocalStorage<string | null>('auth_token', null);
  const { showToast } = useToast();

  const isAuthenticated = !!user && !!token;

  const fetchUser = useCallback(async () => {
    if (!token) {
      setIsLoading(false);
      return;
    }

    try {
      const response = await authApi.getCurrentUser();
      setUser(response.data);
    } catch (error) {
      console.error('Failed to fetch user:', error);
      setToken(null);
      setUser(null);
    } finally {
      setIsLoading(false);
    }
  }, [token, setToken]);

  useEffect(() => {
    fetchUser();
  }, [fetchUser]);

  const login = async (credentials: LoginCredentials): Promise<void> => {
    setIsLoading(true);
    try {
      const response = await authApi.login(credentials);
      const { token: newToken, user: newUser } = response.data;
      
      setToken(newToken);
      setUser(newUser);
      showToast({ type: 'success', message: 'Welcome back!' });
    } catch (error) {
      showToast({ type: 'error', message: 'Invalid credentials' });
      throw error;
    } finally {
      setIsLoading(false);
    }
  };

  const register = async (data: RegisterData): Promise<void> => {
    setIsLoading(true);
    try {
      const response = await authApi.register(data);
      const { token: newToken, user: newUser } = response.data;
      
      setToken(newToken);
      setUser(newUser);
      showToast({ type: 'success', message: 'Account created successfully!' });
    } catch (error) {
      showToast({ type: 'error', message: 'Registration failed' });
      throw error;
    } finally {
      setIsLoading(false);
    }
  };

  const logout = useCallback(() => {
    setToken(null);
    setUser(null);
    showToast({ type: 'info', message: 'Logged out successfully' });
  }, [setToken, showToast]);

  const updateProfile = async (data: Partial<User>): Promise<void> => {
    try {
      const response = await authApi.updateProfile(data);
      setUser(response.data);
      showToast({ type: 'success', message: 'Profile updated' });
    } catch (error) {
      showToast({ type: 'error', message: 'Failed to update profile' });
      throw error;
    }
  };

  const resetPassword = async (email: string): Promise<void> => {
    try {
      await authApi.resetPassword(email);
      showToast({ type: 'success', message: 'Password reset email sent' });
    } catch (error) {
      showToast({ type: 'error', message: 'Failed to send reset email' });
      throw error;
    }
  };

  return {
    user,
    isAuthenticated,
    isLoading,
    login,
    register,
    logout,
    updateProfile,
    resetPassword,
  };
}
