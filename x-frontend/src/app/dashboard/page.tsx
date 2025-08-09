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
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuPortal,
  DropdownMenuSeparator,
  DropdownMenuShortcut,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Button } from "@/components/ui/button";
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
        <div className="flex items-center gap-1 hover:rounded-full p-2 cursor-pointer hover:bg-gray-800 justify-center lg:justify-start">
          <HouseIcon className="text-white" />
          <span className="text-white text-xl font-bold transition-all duration-300 opacity-0 w-0 overflow-hidden lg:opacity-100 lg:w-auto lg:ml-1">
            Home
          </span>
        </div>
        <div className="flex items-center gap-1 hover:rounded-full p-2 cursor-pointer hover:bg-gray-800 justify-center lg:justify-start">
          <SearchIcon className="text-white" />
          <span className="text-white text-xl font-bold  transition-all duration-300 opacity-0 w-0 overflow-hidden lg:opacity-100 lg:w-auto lg:ml-1">
            Explore
          </span>
        </div>
        <div className="flex items-center gap-1 hover:rounded-full p-2 cursor-pointer hover:bg-gray-800 justify-center lg:justify-start">
          <BellIcon className="text-white" />
          <span className="text-white text-xl font-bold  transition-all duration-300 opacity-0 w-0 overflow-hidden lg:opacity-100 lg:w-auto lg:ml-1">
            Notifications
          </span>
        </div>
        <div className="flex items-center gap-1 hover:rounded-full p-2 cursor-pointer hover:bg-gray-800 justify-center lg:justify-start">
          <MailIcon className="text-white" />
          <span className="text-white text-xl font-bold  transition-all duration-300 opacity-0 w-0 overflow-hidden lg:opacity-100 lg:w-auto lg:ml-1">
            Messages
          </span>
        </div>
        <div className="flex items-center gap-1 hover:rounded-full p-2 cursor-pointer hover:bg-gray-800 justify-center lg:justify-start">
          <Image src="/grok.svg" width={30} height={30} alt="" />
          <span className="text-white text-xl font-bold  transition-all duration-300 opacity-0 w-0 overflow-hidden lg:opacity-100 lg:w-auto lg:ml-1">
            Grok
          </span>
        </div>
        <div className="flex items-center gap-1 hover:rounded-full p-2 cursor-pointer hover:bg-gray-800 justify-center lg:justify-start">
          <BookmarkIcon className="text-white" />
          <span className="text-white text-xl font-bold  transition-all duration-300 opacity-0 w-0 overflow-hidden lg:opacity-100 lg:w-auto lg:ml-1">
            Bookmarks
          </span>
        </div>
        <div className="flex items-center gap-1 hover:rounded-full p-2 cursor-pointer hover:bg-gray-800 justify-center lg:justify-start">
          <BriefcaseBusinessIcon className="text-white" />
          <span className="text-white text-xl font-bold  transition-all duration-300 opacity-0 w-0 overflow-hidden lg:opacity-100 lg:w-auto lg:ml-1">
            Jobs
          </span>
        </div>
        <div className="flex items-center gap-1 hover:rounded-full p-2 cursor-pointer hover:bg-gray-800 justify-center lg:justify-start">
          <UsersIcon className="text-white" />
          <span className="text-white text-xl font-bold  transition-all duration-300 opacity-0 w-0 overflow-hidden lg:opacity-100 lg:w-auto lg:ml-1">
            Communities
          </span>
        </div>
        <div className="flex items-center gap-1 hover:rounded-full p-2 cursor-pointer hover:bg-gray-800 justify-center lg:justify-start">
          <Image src="/icon.svg" width={25} height={25} alt="" />
          <span className="text-white text-xl font-bold  transition-all duration-300 opacity-0 w-0 overflow-hidden lg:opacity-100 lg:w-auto lg:ml-1">
            Premium
          </span>
        </div>
        <div className="flex items-center gap-1 hover:rounded-full p-2 cursor-pointer hover:bg-gray-800 justify-center lg:justify-start">
          <ZapIcon className="text-white" />
          <span className="text-white text-xl font-bold  transition-all duration-300 opacity-0 w-0 overflow-hidden lg:opacity-100 lg:w-auto lg:ml-1">
            Verified Orgs
          </span>
        </div>
        <div className="flex items-center gap-1 hover:rounded-full p-2 cursor-pointer hover:bg-gray-800 justify-center lg:justify-start">
          <UserIcon className="text-white" />
          <span className="text-white text-xl font-bold  transition-all duration-300 opacity-0 w-0 overflow-hidden lg:opacity-100 lg:w-auto lg:ml-1">
            Profile
          </span>
        </div>
        <div className="flex items-center gap-1 hover:rounded-full p-2 cursor-pointer hover:bg-gray-800 justify-center lg:justify-start">
          <CircleEllipsisIcon className="text-white" />
          <span className="text-white text-xl font-bold  transition-all duration-300 opacity-0 w-0 overflow-hidden lg:opacity-100 lg:w-auto lg:ml-1">
            More
          </span>
        </div>
        <div className="flex items-center gap-1  p-2 cursor-pointer  justify-center lg:justify-start">
          <PenLineIcon
            className="text-white inline lg:hidden hover:bg-gray-800
            hover:rounded-full"
          />
          <button className="bg-white text-black rounded-full w-full h-10 font-bold hidden lg:inline cursor-pointer">
            Post
          </button>
        </div>
        <div className="flex items-center cursor-pointer hover:rounded-full p-3  hover:bg-gray-800 ">
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <div className="flex items-center justify-between gap-2">
                <Image src="/icon.svg" width={30} height={30} alt="" />
                <div className="flex-col hidden lg:flex ">
                  <span className="text-white text-sm">Yash</span>
                  <span className="text-gray-400 text-sm">@https_200</span>
                </div>
                <button className="flex-col hidden lg:flex text-white cursor-pointer outline-none">
                  ...
                </button>
              </div>
            </DropdownMenuTrigger>
            <DropdownMenuContent className="w-56" align="start">
              <DropdownMenuLabel>My Account</DropdownMenuLabel>
              <DropdownMenuGroup>
                <DropdownMenuItem>
                  Profile
                  <DropdownMenuShortcut>⇧⌘P</DropdownMenuShortcut>
                </DropdownMenuItem>
                <DropdownMenuItem>
                  Billing
                  <DropdownMenuShortcut>⌘B</DropdownMenuShortcut>
                </DropdownMenuItem>
                <DropdownMenuItem>
                  Settings
                  <DropdownMenuShortcut>⌘S</DropdownMenuShortcut>
                </DropdownMenuItem>
                <DropdownMenuItem>
                  Keyboard shortcuts
                  <DropdownMenuShortcut>⌘K</DropdownMenuShortcut>
                </DropdownMenuItem>
              </DropdownMenuGroup>
              <DropdownMenuSeparator />
              <DropdownMenuGroup>
                <DropdownMenuItem>Team</DropdownMenuItem>
                <DropdownMenuSub>
                  <DropdownMenuSubTrigger>Invite users</DropdownMenuSubTrigger>
                  <DropdownMenuPortal>
                    <DropdownMenuSubContent>
                      <DropdownMenuItem>Email</DropdownMenuItem>
                      <DropdownMenuItem>Message</DropdownMenuItem>
                      <DropdownMenuSeparator />
                      <DropdownMenuItem>More...</DropdownMenuItem>
                    </DropdownMenuSubContent>
                  </DropdownMenuPortal>
                </DropdownMenuSub>
                <DropdownMenuItem>
                  New Team
                  <DropdownMenuShortcut>⌘+T</DropdownMenuShortcut>
                </DropdownMenuItem>
              </DropdownMenuGroup>
              <DropdownMenuSeparator />
              <DropdownMenuItem>GitHub</DropdownMenuItem>
              <DropdownMenuItem>Support</DropdownMenuItem>
              <DropdownMenuItem disabled>API</DropdownMenuItem>
              <DropdownMenuSeparator />
              <DropdownMenuItem>
                Log out
                <DropdownMenuShortcut>⇧⌘Q</DropdownMenuShortcut>
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      </div>
      <div className="flex flex-1 w-screen justify-between border-l-[1px] border-r-[1px] border-gray-800">
        <div className="flex mt-1 flex-1 h-12 justify-between text-white border-b-[1px] border-gray-800">
          <span className="flex-1 hover:bg-gray-800 justify-center items-center flex font-bold cursor-pointer">
            <span className="border-b-[2px] border-blue-400 pb-2">For you</span>
          </span>
          <span className="flex-1 hover:bg-gray-800 justify-center items-center flex font-bold cursor-pointer">
            <span className="border-b-[2px] border-blue-400 pb-2">
              Following
            </span>
          </span>
        </div>
      </div>
      <div className="flex-1">1</div>
    </div>
  );
};

export default Dashboard;
