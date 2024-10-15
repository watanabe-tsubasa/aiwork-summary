'use client';
import { SetStateAction } from "react";
import * as Switch from "@radix-ui/react-switch";

interface BaseSwitchProps {
  label: string;
  checked: boolean;
  onCheckedChange: React.Dispatch<SetStateAction<boolean>>;
}

const BaseSwitch: React.FC<BaseSwitchProps> = ({ label, checked, onCheckedChange }) => (
	<form>
		<div className="flex flex-col items-center space-y-2">
			<label
				className="text-[15px] leading-none text-slate-800"
			>
				{label}
			</label>
			<Switch.Root
				className="relative h-[25px] w-[42px] cursor-default rounded-full bg-blackA6 outline-none shadow-[0_0_0_2px] shadow-black data-[state=checked]:bg-black"
        checked={checked}
        onCheckedChange={onCheckedChange}
			>
				<Switch.Thumb
         className="block size-[21px] translate-x-0.5 rounded-full bg-white shadow-[0_2px_2px] shadow-blackA4 transition-transform duration-100 will-change-transform data-[state=checked]:translate-x-[19px]"
        />
			</Switch.Root>
		</div>
	</form>
);

export default BaseSwitch;
