import React from "react";
import Image from "next/image";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

export function Login() {
  const handleLogin = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    const formData = new FormData(e.currentTarget);

    const email = formData.get("email");
    const password = formData.get("password");

    fetch("http://localhost:8080/auth/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
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
    <Dialog>
      <DialogTrigger asChild>
        <button className="flex items-center bg-white rounded-full p-2 mt-4 w-80 justify-center gap-2 cursor-pointer">
          <span className="text-blue-400 font-semibold">Sign in</span>
        </button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-[425px]">
        <form onSubmit={handleLogin}>
          <DialogHeader>
            <DialogTitle className="flex items-center justify-center">
              <Image src="/x-white.svg" width={30} height={30} alt="" />
            </DialogTitle>
            <DialogDescription className="text-2xl font-bold text-black">
              Sign in to your account
            </DialogDescription>
          </DialogHeader>
          <div className="grid gap-4">
            <div className="grid gap-3">
              <Label htmlFor="email-1">Email</Label>
              <Input
                id="email-1"
                name="email"
                placeholder="john@doe.com"
                type="email"
              />
            </div>
            <div className="grid gap-3">
              <Label htmlFor="password-1">Password</Label>
              <Input
                id="password-1"
                name="password"
                placeholder="********"
                type="password"
              />
            </div>
          </div>

          <DialogFooter className="mt-6">
            <DialogClose asChild>
              <Button variant="outline">Cancel</Button>
            </DialogClose>
            <Button type="submit">Login</Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
