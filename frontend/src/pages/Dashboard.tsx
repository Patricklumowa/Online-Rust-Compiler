import React, { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import axios from 'axios';
import { Plus, Trash2, Edit, Code } from 'lucide-react';

interface Snippet {
  id: number;
  title: string;
  code: string;
  language: string;
  created_at: string;
  updated_at: string;
}

const Dashboard: React.FC = () => {
  const [snippets, setSnippets] = useState<Snippet[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');

  useEffect(() => {
    fetchSnippets();
  }, []);

  const fetchSnippets = async () => {
    try {
      const response = await axios.get('http://localhost:3001/snippets');
      setSnippets(response.data);
      setLoading(false);
    } catch (err) {
      setError('Failed to fetch snippets');
      setLoading(false);
    }
  };

  const handleDelete = async (id: number) => {
    if (!window.confirm('Are you sure you want to delete this snippet?')) return;
    
    try {
      await axios.delete(`http://localhost:3001/snippets/${id}`);
      setSnippets(snippets.filter(s => s.id !== id));
    } catch (err) {
      alert('Failed to delete snippet');
    }
  };

  if (loading) return <div className="text-white text-center mt-10">Loading...</div>;

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="flex justify-between items-center mb-8">
        <h1 className="text-3xl font-bold text-white">My Snippets</h1>
        <Link 
          to="/editor" 
          className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-md flex items-center gap-2 transition-colors"
        >
          <Plus className="w-4 h-4" />
          New Snippet
        </Link>
      </div>

      {error && (
        <div className="bg-red-500/10 border border-red-500/50 text-red-500 p-4 rounded mb-6">
          {error}
        </div>
      )}

      {snippets.length === 0 ? (
        <div className="text-center py-12 bg-gray-900 rounded-lg border border-gray-800">
          <Code className="w-12 h-12 text-gray-600 mx-auto mb-4" />
          <h3 className="text-xl font-medium text-gray-300 mb-2">No snippets yet</h3>
          <p className="text-gray-500 mb-6">Create your first Rust code snippet to get started.</p>
          <Link 
            to="/editor" 
            className="text-blue-400 hover:text-blue-300 font-medium"
          >
            Create Snippet
          </Link>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {snippets.map((snippet) => (
            <div key={snippet.id} className="bg-gray-900 border border-gray-800 rounded-lg p-6 hover:border-gray-700 transition-colors">
              <div className="flex justify-between items-start mb-4">
                <h3 className="text-xl font-semibold text-white truncate pr-4">{snippet.title}</h3>
                <span className="text-xs bg-gray-800 text-gray-400 px-2 py-1 rounded uppercase">
                  {snippet.language}
                </span>
              </div>
              
              <div className="bg-gray-950 rounded p-3 mb-4 h-32 overflow-hidden relative">
                <div className="absolute inset-0 bg-gradient-to-b from-transparent to-gray-950/90 pointer-events-none" />
                <pre className="text-gray-400 text-xs font-mono">
                  {snippet.code}
                </pre>
              </div>
              
              <div className="flex justify-between items-center text-sm text-gray-500 mt-4 pt-4 border-t border-gray-800">
                <span>{new Date(snippet.updated_at).toLocaleDateString()}</span>
                <div className="flex gap-3">
                  <Link 
                    to={`/editor/${snippet.id}`}
                    className="text-blue-400 hover:text-blue-300 flex items-center gap-1"
                  >
                    <Edit className="w-4 h-4" />
                    Edit
                  </Link>
                  <button 
                    onClick={() => handleDelete(snippet.id)}
                    className="text-red-400 hover:text-red-300 flex items-center gap-1"
                  >
                    <Trash2 className="w-4 h-4" />
                    Delete
                  </button>
                </div>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default Dashboard;
