"use client";

import { ReactHTMLElement } from "react";

export default function Home() {
  const handleLogin = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    const formData = new FormData(e.currentTarget);
    const username = formData.get("username");
    const email = formData.get("email");
    const password = formData.get("password");

    // Here you would typically send the data to your backend for authentication
    console.log("Username:", username);
    console.log("Email:", email);
    console.log("Password:", password);

    fetch("http://localhost:8080/auth/register", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        username,
        email,
        password,
      }),
    })
      .then((response) => {
        if (!response.ok) {
          throw new Error("Network response was not ok");
        }
        return response.json();
      })
      .then((data) => {
        console.log("Login successful:", data);
      })
      .catch((error) => {
        console.error("There was a problem with the login request:", error);
      });
  };

  return (
    <div className="flex items-center justify-center h-screen flex-col">
      <div className="flex h-100 w-100">
        <form onSubmit={handleLogin} className="flex flex-col mx-auto gap-2">
          <div className="flex flex-col ">
            <label htmlFor="">Username</label>
            <input
              type="text"
              name="username"
              id="username"
              className="border border-gray-300 p-2 rounded-md"
            />
          </div>
          <div className="flex flex-col ">
            <label htmlFor="">Email</label>
            <input
              type="email"
              name="email"
              id="email"
              className="border border-gray-300 p-2 rounded-md"
            />
          </div>
          <div className="flex flex-col">
            <label htmlFor="">Password</label>
            <input
              type="password"
              name="password"
              id="password"
              className="border border-gray-300 p-2 rounded-md"
            />
          </div>
          <button
            type="submit"
            className="bg-black text-white text-sm p-2 rounded-full"
          >
            Login
          </button>
        </form>
      </div>
    </div>
  );
}
