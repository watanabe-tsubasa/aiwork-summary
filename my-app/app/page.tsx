'use client';

import Image from "next/image";
import { useState } from "react";
import { invoke } from '@tauri-apps/api/tauri';

export default function Home() {
  const [filePath, setFilePath] = useState<string | null>(null);
  const [storeName, setStoreName] = useState<string[] | string>("店舗が選ばれていません");

  const handleFileSelect = async () => {
    try {
      const path = await invoke<string>('select_excel_file');
      setFilePath(path);

      const excelData: string[] | string = await invoke('read_excel_file', { path });
      setStoreName(excelData)
    } catch (error) {
      console.error(error);
    }    
  }
  return (
    <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
      <main className="flex flex-col gap-8 row-start-2 items-center">
        <h1 className="text-4xl font-bold">AI work summarizer</h1>
        <div className="flex gap-x-4 items-end">
          <Image
            className="dark:invert"
            src="https://nextjs.org/icons/next.svg"
            alt="Next.js logo"
            width={180}
            height={38}
            priority
          />
          <p className="text-2xl">and</p>
          <Image
            className="dark:invert"
            src="/header_light.svg"
            alt="tauri logo"
            width={120}
            height={38}
          />
        </div>
        <ol className="list-inside list-decimal text-sm text-left font-[family-name:var(--font-geist-mono)]">
          <li className="mb-2">
            <code className="bg-black/[.05] dark:bg-white/[.06] px-1 py-0.5 rounded font-semibold">
              upload file
            </code>
            {" "} button and select {" "}
            <code className="bg-black/[.05] dark:bg-white/[.06] px-1 py-0.5 rounded font-semibold">
              .xlsx
            </code>
            {" "}file
          </li>
          <li className="mb-2">
            push {" "}
            <code className="bg-black/[.05] dark:bg-white/[.06] px-1 py-0.5 rounded font-semibold">
              get Data
            </code>
            {" "} button
          </li>
          <li>wait for a minute...</li>
        </ol>

        <div className="flex gap-4 items-center flex-col sm:flex-row">
          <button
            className="rounded-full border border-solid border-transparent transition-colors flex items-center justify-center bg-foreground text-background gap-2 hover:bg-[#383838] dark:hover:bg-[#ccc] text-sm sm:text-base h-10 sm:h-12 px-4 min-w-44"
            onClick={handleFileSelect}
          >
            upload file
          </button>
          <button
            className="rounded-full border border-solid border-black/[.08] dark:border-white/[.145] transition-colors flex items-center justify-center hover:bg-[#f2f2f2] dark:hover:bg-[#1a1a1a] hover:border-transparent text-sm sm:text-base h-10 sm:h-12 px-4 sm:px-5 min-w-44"
          >
            get Data 
          </button>

        </div>
        <div>
          <p>{filePath && `file name:${filePath}`}</p>
        </div>
        <div>
          {typeof storeName === 'string' ? (
            <p>{storeName}</p>
          ) : Array.isArray(storeName) ? (
            <div className="flex flex-col items-center">
              <h2 className="pb-4 text-2xl">店舗名</h2>
              <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
                {storeName.map((item, index) => (
                  <div
                    key={index}
                    className="bg-white rounded-lg shadow-md overflow-hidden transition-all duration-300 hover:shadow-lg hover:-translate-y-1 border border-gray-200 px-2 py-1"
                  >
                    <p className="text- font-medium text-center text-gray-800">{item}</p>
                  </div>
                ))}
              </div>
            </div>
          ) : null}
        </div>
      </main>
      <footer className="row-start-3 flex gap-6 flex-wrap items-center justify-center">
        <div>
          <p>made in 2024</p>
        </div>
      </footer>
    </div>
  );
}
