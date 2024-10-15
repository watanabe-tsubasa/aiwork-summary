'use client';

import Image from "next/image";
import { useReducer, useState } from "react";
import { invoke } from '@tauri-apps/api/tauri';
import { initialState, reducer } from "./reducer";
import BaseSwitch from "./Switch";

export default function Home() {
  const [filePath, setFilePath] = useState<string | null>(null);
  const [storeNames, dispatch] = useReducer(reducer, initialState);
  const [hasError, setHasError] = useState<boolean>(false);
  const [inputText, setInputText] = useState<string>('');
  const [isHeadless, setIsHeadless] = useState<boolean>(false);

  const handleFileSelect = async () => {
    try {
      setHasError(false);
      const path = await invoke<string>('select_excel_file');
      setFilePath(path);
      const excelData: string[] | string = await invoke('read_excel_file', { path });
      if (Array.isArray(excelData)) {
        dispatch({'type': 'setAll', 'storeList': excelData});
      } else {
        setHasError(true);
      }  
    } catch (error) {
      console.error(error);
    }    
  }

  const handleFileOutput = async () => {
    try {
      await invoke('scraper', { storeNames: storeNames, headless: !isHeadless });
      dispatch({'type': 'reset'});
    } catch (error) {
      console.error(error)
    }
  }

  const handleInput = (e: React.ChangeEvent<HTMLInputElement>) => {
    setInputText(e.target.value);
  }

  const handleClickAddButton = () => {
    if(inputText.length === 0) return
    dispatch({'type': 'add', 'store': inputText})
    setInputText('');
  }

  return (
    <div className="grid items-center justify-items-center min-h-screen p-8 pb-20 gap-16 font-[family-name:var(--font-geist-sans)]">
      <main className="flex flex-col gap-8 row-start-2 items-center">
        <div className="flex flex-row items-end justify-center space-x-6">
          <h1 className="text-4xl font-bold">AI work summarizer</h1>
          <BaseSwitch
            label="debugger"
            checked={isHeadless}
            onCheckedChange={setIsHeadless}
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
            onClick={handleFileOutput}
          >
            get Data 
          </button>
        </div>
        <div className="max-w-md mx-auto p-2">
          <div className="flex flex-col sm:flex-row gap-6">
            <input
              type="text"
              placeholder="個別追加店舗"
              className="flex-grow px-4 py-2 text-gray-700 bg-white border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition duration-200 ease-in-out"
              aria-label="個別店舗入力"
              value={inputText}
              onChange={handleInput}   
            />
            <button
              type='button'
              className="px-6 py-2 text-white bg-gray-600 rounded-full hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2 transition duration-200 ease-in-out"
              onClick={handleClickAddButton}
              disabled={inputText.length === 0}
            >
              add
            </button>
          </div>
        </div>
        <div>
          <p>{filePath && `file name:${filePath}`}</p>
        </div>
        <div>
          {storeNames ? (
            <div className="flex flex-col items-center">
              <h2 className="pb-4 text-2xl">店舗名</h2>
              <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
                {storeNames.map((item, index) => (
                  <button
                    key={index}
                    type='button'
                    onClick={() => dispatch({'type': 'deleate', 'index': index})}
                    className="bg-white rounded-lg shadow-md overflow-hidden transition-all duration-300 hover:shadow-lg hover:-translate-y-1 border border-gray-200 px-2 py-1"
                  >
                    <p className="text- font-medium text-center text-gray-800">{item}</p>
                  </button>
                ))}
              </div>
            </div>
          ) : <p>店舗が選ばれていません</p>}
          {hasError && <p className="text-red-800">読み込みエラーが発生しました</p>}
        </div>
      </main>
      <footer className="row-start-3 flex gap-6 flex-col items-center justify-center">
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
        <div>
          <p>made in 2024</p>
        </div>
      </footer>
    </div>
  );
}
