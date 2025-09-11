'use client';
import { useEffect, useState } from 'react';

export default function Home() {
  const [msg, setMsg] = useState('');
  useEffect(() => {
    fetch('http://localhost:8080/api/ping')
      .then(res => res.json())
      .then(data => setMsg(data.message));
  }, []);
  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-24">
      <h1 className="text-4xl font-bold">Welcome to Harmonify</h1>
      <p>Backend says: {msg}</p>
    </main>
  );
}