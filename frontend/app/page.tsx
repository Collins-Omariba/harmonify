'use client';
import { useEffect, useState } from 'react';
import Link from 'next/link';

export default function Home() {
  const [msg, setMsg] = useState('');
  const [isLoggedIn, setIsLoggedIn] = useState(false);

  useEffect(() => {
    // Check if user is logged in
    const token = localStorage.getItem('token');
    setIsLoggedIn(!!token);

    // Test backend connection
    fetch(`${process.env.NEXT_PUBLIC_API_BASE_URL}/`)
      .then(() => setMsg('Backend connected successfully'))
      .catch(() => setMsg('Backend connection failed'));
  }, []);

  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-24">
      <h1 className="text-4xl font-bold mb-8">Welcome to Harmonify</h1>
      <p className="mb-8 text-lg text-gray-600">Your personal productivity and wellness companion</p>
      
      <div className="mb-8">
        <p className="text-sm text-gray-500">Status: {msg}</p>
      </div>

      <div className="flex gap-4">
        {isLoggedIn ? (
          <Link 
            href="/dashboard" 
            className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
          >
            Go to Dashboard
          </Link>
        ) : (
          <>
            <Link 
              href="/login" 
              className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
            >
              Login
            </Link>
            <Link 
              href="/register" 
              className="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded"
            >
              Register
            </Link>
          </>
        )}
      </div>
    </main>
  );
}