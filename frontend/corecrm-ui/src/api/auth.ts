import { apiClient } from './client';
import { LoginRequest, LoginResponse, User } from '../types';

export const authApi = {
  login: (credentials: LoginRequest) =>
    apiClient.post<LoginResponse>('/api/v1/auth/login', credentials),

  getMe: () =>
    apiClient.get<User>('/api/v1/auth/me'),

  logout: () => {
    localStorage.removeItem('access_token');
    localStorage.removeItem('user');
  },
};
