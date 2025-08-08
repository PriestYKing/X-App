"use client";

import Image from "next/image";
import React, { ReactHTMLElement } from "react";
import { Register } from "./components/Register";
import { Login } from "./components/Login";

export default function Home() {
  const [open, setOpen] = React.useState(false);
  const [date, setDate] = React.useState<Date | undefined>(undefined);
  return (
    <div className="flex items-center justify-center h-screen flex-col bg-black">
      <div>
        <Image src="/icon.svg" width={30} height={30} alt="" />
      </div>
      <span className="text-2xl text-white font-bold p-8">
        Create an account
      </span>
      <button className="flex items-center bg-white rounded-full p-2 w-80 justify-center gap-2 border border-blue-300 cursor-pointer">
        <Image src="/google.svg" width={30} height={30} alt="" />
        <span className="text-black font-semibold">Sign up with Google</span>
      </button>
      <button className="flex items-center bg-white rounded-full p-2 mt-4 w-80 justify-center gap-2 cursor-pointer">
        <Image src="/apple.svg" width={30} height={30} alt="" />
        <span className="text-black font-semibold">Sign up with Apple</span>
      </button>
      <div className="flex gap-20 mt-8 items-center h-[1px]">
        <div className="bg-gray-400 w-[1px] h-30 rotate-90"></div>
        <div className="text-white">OR</div>
        <div className="bg-gray-400 w-[1px] h-30 rotate-90"></div>
      </div>

      <div className="">
        <Register open={open} setOpen={setOpen} date={date} setDate={setDate} />
      </div>
      <span className="text-gray-400 text-xs mt-4 w-80 items-center text-center">
        By signing up, you agree to the{" "}
        <a href="" className="text-blue-400 hover:underline">
          Terms of Service
        </a>{" "}
        and{" "}
        <a href="" className="text-blue-400 hover:underline">
          Privacy Policy{" "}
        </a>
        , including{" "}
        <a href="" className="text-blue-400 hover:underline">
          Cookie Use
        </a>
        .
      </span>
      <span className="text-white font-semibold mt-8">
        Already have an account?
      </span>
      <Login />
      <button className="flex items-center rounded-full p-2 mt-4 w-80 justify-center gap-2 cursor-pointer border border-gray-300">
        <Image src="/grok.svg" width={30} height={30} alt="" />
        <span className="text-white font-semibold">Get Grok</span>
      </button>
    </div>
  );
}
