"use client";

import {
  BellIcon,
  BookmarkIcon,
  BriefcaseBusinessIcon,
  CircleEllipsisIcon,
  HouseIcon,
  MailIcon,
  PenLineIcon,
  SearchIcon,
  UserIcon,
  UsersIcon,
  ZapIcon,
} from "lucide-react";
import Image from "next/image";

const Dashboard = () => {
  return (
    <div className="flex h-screen w-full justify-between bg-black">
      <div className="flex flex-col mt-1 gap-0.5 px-4 lg:px-20">
        <div className="flex justify-center lg:justify-start">
          <Image
            src="/icon.svg"
            width={60}
            height={60}
            alt=""
            className="hover:rounded-full p-3 cursor-pointer hover:bg-gray-800"
          />
        </div>
        <div className="flex items-center gap-2 hover:rounded-full p-3 cursor-pointer hover:bg-gray-800 justify-center lg:justify-start">
          <HouseIcon className="text-white" />
          <span className="text-white text-xl font-bold transition-all duration-300 opacity-0 w-0 overflow-hidden lg:opacity-100 lg:w-auto lg:ml-1">
            Home
          </span>
        </div>
        <div className="flex items-center gap-2 hover:rounded-full p-3 cursor-pointer hover:bg-gray-800 justify-center lg:justify-start">
          <SearchIcon className="text-white" />
          <span className="text-white text-xl font-bold  transition-all duration-300 opacity-0 w-0 overflow-hidden lg:opacity-100 lg:w-auto lg:ml-1">
            Explore
          </span>
        </div>
        <div className="flex items-center gap-2 hover:rounded-full p-3 cursor-pointer hover:bg-gray-800 justify-center lg:justify-start">
          <BellIcon className="text-white" />
          <span className="text-white text-xl font-bold  transition-all duration-300 opacity-0 w-0 overflow-hidden lg:opacity-100 lg:w-auto lg:ml-1">
            Notifications
          </span>
        </div>
        <div className="flex items-center gap-2 hover:rounded-full p-3 cursor-pointer hover:bg-gray-800 justify-center lg:justify-start">
          <MailIcon className="text-white" />
          <span className="text-white text-xl font-bold  transition-all duration-300 opacity-0 w-0 overflow-hidden lg:opacity-100 lg:w-auto lg:ml-1">
            Messages
          </span>
        </div>
        <div className="flex items-center gap-2 hover:rounded-full p-3 cursor-pointer hover:bg-gray-800 justify-center lg:justify-start">
          <Image src="/grok.svg" width={30} height={30} alt="" />
          <span className="text-white text-xl font-bold  transition-all duration-300 opacity-0 w-0 overflow-hidden lg:opacity-100 lg:w-auto lg:ml-1">
            Grok
          </span>
        </div>
        <div className="flex items-center gap-2 hover:rounded-full p-3 cursor-pointer hover:bg-gray-800 justify-center lg:justify-start">
          <BookmarkIcon className="text-white" />
          <span className="text-white text-xl font-bold  transition-all duration-300 opacity-0 w-0 overflow-hidden lg:opacity-100 lg:w-auto lg:ml-1">
            Bookmarks
          </span>
        </div>
        <div className="flex items-center gap-2 hover:rounded-full p-3 cursor-pointer hover:bg-gray-800 justify-center lg:justify-start">
          <BriefcaseBusinessIcon className="text-white" />
          <span className="text-white text-xl font-bold  transition-all duration-300 opacity-0 w-0 overflow-hidden lg:opacity-100 lg:w-auto lg:ml-1">
            Jobs
          </span>
        </div>
        <div className="flex items-center gap-2 hover:rounded-full p-3 cursor-pointer hover:bg-gray-800 justify-center lg:justify-start">
          <UsersIcon className="text-white" />
          <span className="text-white text-xl font-bold  transition-all duration-300 opacity-0 w-0 overflow-hidden lg:opacity-100 lg:w-auto lg:ml-1">
            Communities
          </span>
        </div>
        <div className="flex items-center gap-2 hover:rounded-full p-3 cursor-pointer hover:bg-gray-800 justify-center lg:justify-start">
          <Image src="/icon.svg" width={25} height={25} alt="" />
          <span className="text-white text-xl font-bold  transition-all duration-300 opacity-0 w-0 overflow-hidden lg:opacity-100 lg:w-auto lg:ml-1">
            Premium
          </span>
        </div>
        <div className="flex items-center gap-2 hover:rounded-full p-3 cursor-pointer hover:bg-gray-800 justify-center lg:justify-start">
          <ZapIcon className="text-white" />
          <span className="text-white text-xl font-bold  transition-all duration-300 opacity-0 w-0 overflow-hidden lg:opacity-100 lg:w-auto lg:ml-1">
            Verified Orgs
          </span>
        </div>
        <div className="flex items-center gap-2 hover:rounded-full p-3 cursor-pointer hover:bg-gray-800 justify-center lg:justify-start">
          <UserIcon className="text-white" />
          <span className="text-white text-xl font-bold  transition-all duration-300 opacity-0 w-0 overflow-hidden lg:opacity-100 lg:w-auto lg:ml-1">
            Profile
          </span>
        </div>
        <div className="flex items-center gap-2 hover:rounded-full p-3 cursor-pointer hover:bg-gray-800 justify-center lg:justify-start">
          <CircleEllipsisIcon className="text-white" />
          <span className="text-white text-xl font-bold  transition-all duration-300 opacity-0 w-0 overflow-hidden lg:opacity-100 lg:w-auto lg:ml-1">
            More
          </span>
        </div>
        <div className="flex items-center gap-2  p-3 cursor-pointer  justify-center lg:justify-start">
          <PenLineIcon
            className="text-white inline lg:hidden hover:bg-gray-800
            hover:rounded-full"
          />
          <button className="bg-white text-black rounded-full w-full h-10 font-bold hidden lg:inline">
            Post
          </button>
        </div>
      </div>
      <div className="flex-1">1</div>
      <div className="flex-1">1</div>
    </div>
  );
};

export default Dashboard;
