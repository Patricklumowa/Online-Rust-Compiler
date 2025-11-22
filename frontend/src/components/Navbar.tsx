import React from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { useAuth } from '../context/AuthContext';
import { LogOut, Code, LayoutDashboard, User } from 'lucide-react';

const Navbar: React.FC = () => {
  const { user, logout, isAuthenticated } = useAuth();
  const navigate = useNavigate();

  const handleLogout = () => {
    logout();
    navigate('/login');
  };

  return (
    <nav className="bg-gray-900 border-b border-gray-800 p-4">
      <div className="container mx-auto flex justify-between items-center">
        <Link to="/" className="text-xl font-bold text-white flex items-center gap-2">
          <Code className="w-6 h-6 text-blue-500" />
          Rust Compiler
        </Link>
        
        <div className="flex items-center gap-6">
          {isAuthenticated ? (
            <>
              <Link to="/dashboard" className="text-gray-300 hover:text-white flex items-center gap-2">
                <LayoutDashboard className="w-4 h-4" />
                Dashboard
              </Link>
              <Link to="/editor" className="text-gray-300 hover:text-white flex items-center gap-2">
                <Code className="w-4 h-4" />
                New Snippet
              </Link>
              <div className="flex items-center gap-4 ml-4 border-l border-gray-700 pl-4">
                <span className="text-gray-400 flex items-center gap-2">
                  <User className="w-4 h-4" />
                  {user?.username}
                </span>
                <button 
                  onClick={handleLogout}
                  className="text-red-400 hover:text-red-300 flex items-center gap-1 text-sm"
                >
                  <LogOut className="w-4 h-4" />
                  Logout
                </button>
              </div>
            </>
          ) : (
            <div className="flex gap-4">
              <Link to="/login" className="text-gray-300 hover:text-white">Login</Link>
              <Link to="/register" className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-md transition-colors">
                Register
              </Link>
            </div>
          )}
        </div>
      </div>
    </nav>
  );
};

export default Navbar;
