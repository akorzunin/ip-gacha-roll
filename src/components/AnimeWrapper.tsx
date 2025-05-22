import { ReactNode } from "react";
import tao from "../assets/tao.png";
import qiqi from "../assets/qiqi.png";
import working from "../assets/working.png";

export const AnimeWrapper: React.FC<{ children: ReactNode }> = ({
  children,
}) => {
  return (
    <div className="relative overflow-hidden">
      <img
        src={tao}
        alt="Tao"
        className="absolute top-25 -left-30 z-10 h-120 rotate-45"
      />
      <img
        src={qiqi}
        alt="Qiqi"
        className="absolute top-50 -right-8 z-30 h-40 w-40 rotate-270 transition-transform duration-300 ease-in-out hover:translate-x-12"
      />
      <img
        src={working}
        alt="Working"
        className="absolute -right-5 -bottom-0 z-10 h-40"
      />
      {children}
    </div>
  );
};
