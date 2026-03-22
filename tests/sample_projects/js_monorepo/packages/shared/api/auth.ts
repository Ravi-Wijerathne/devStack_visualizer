/**
 * API client for authentication endpoints
 */

import { apiClient } from './client';
import type { User, LoginCredentials, RegisterData, AuthResponse } from '../types';

class AuthApi {
  private readonly baseUrl = '/api/v1/auth';

  async login(credentials: LoginCredentials): Promise<AuthResponse> {
    const response = await apiClient.post<AuthResponse>(
      `${this.baseUrl}/login`,
      credentials
    );
    return response.data;
  }

  async register(data: RegisterData): Promise<AuthResponse> {
    const response = await apiClient.post<AuthResponse>(
      `${this.baseUrl}/register`,
      data
    );
    return response.data;
  }

  async logout(): Promise<void> {
    await apiClient.post(`${this.baseUrl}/logout`);
  }

  async getCurrentUser(): Promise<{ data: User }> {
    const response = await apiClient.get<{ data: User }>(
      `${this.baseUrl}/me`
    );
    return response.data;
  }

  async updateProfile(data: Partial<User>): Promise<{ data: User }> {
    const response = await apiClient.patch<{ data: User }>(
      `${this.baseUrl}/profile`,
      data
    );
    return response.data;
  }

  async resetPassword(email: string): Promise<void> {
    await apiClient.post(`${this.baseUrl}/reset-password`, { email });
  }

  async verifyEmail(token: string): Promise<void> {
    await apiClient.post(`${this.baseUrl}/verify-email`, { token });
  }

  async refreshToken(refreshToken: string): Promise<{ token: string }> {
    const response = await apiClient.post<{ token: string }>(
      `${this.baseUrl}/refresh`,
      { refreshToken }
    );
    return response.data;
  }
}

export const authApi = new AuthApi();
