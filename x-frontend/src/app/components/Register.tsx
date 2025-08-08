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
import { Calendar } from "@/components/ui/calendar";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import { ChevronDownIcon } from "lucide-react";

type Props = {
  open: boolean;
  setOpen: (open: boolean) => void;
  date: Date | undefined;
  setDate: (date: Date | undefined) => void;
};

export function Register({ open, setOpen, date, setDate }: Props) {
  const handleRegister = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    const formData = new FormData(e.currentTarget);
    const username = formData.get("name");
    const email = formData.get("email");
    const password = formData.get("password");

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
    <Dialog>
      <DialogTrigger asChild>
        <button className="flex items-center bg-blue-400 rounded-full p-2 mt-8 w-80 justify-center gap-2 cursor-pointer">
          <span className="text-white font-semibold">Create Account</span>
        </button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-[425px]">
        <form onSubmit={handleRegister}>
          <DialogHeader>
            <DialogTitle className="flex items-center justify-center">
              <Image src="/x-white.svg" width={30} height={30} alt="" />
            </DialogTitle>
            <DialogDescription className="text-2xl font-bold text-black">
              Create your account
            </DialogDescription>
          </DialogHeader>
          <div className="grid gap-4">
            <div className="grid gap-3">
              <Label htmlFor="name-1">Name</Label>
              <Input
                id="name-1"
                name="name"
                type="text"
                placeholder="John Doe"
              />
            </div>
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
          <div className="flex flex-col mt-4">
            <span className="font-semibold">Date of birth</span>
            <span className="text-gray-500 text-xs">
              This will not be shown publicly. Confirm your own age, even if
              this account is for a business, a pet, or something else.
            </span>
            <div className="mt-4">
              <Popover open={open} onOpenChange={setOpen}>
                <PopoverTrigger asChild>
                  <Button
                    variant="outline"
                    id="date"
                    className="w-48 justify-between font-normal"
                  >
                    {date ? date.toLocaleDateString() : "Select date"}
                    <ChevronDownIcon />
                  </Button>
                </PopoverTrigger>
                <PopoverContent
                  className="w-auto overflow-hidden p-0"
                  align="start"
                >
                  <Calendar
                    mode="single"
                    selected={date}
                    captionLayout="dropdown"
                    onSelect={(date) => {
                      setDate(date);
                      setOpen(false);
                    }}
                  />
                </PopoverContent>
              </Popover>
            </div>
          </div>
          <DialogFooter className="mt-6">
            <DialogClose asChild>
              <Button variant="outline">Cancel</Button>
            </DialogClose>
            <Button type="submit">Sign Up</Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
