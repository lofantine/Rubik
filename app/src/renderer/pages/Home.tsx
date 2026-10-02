import { Link } from "react-router";

const GITHUB_URL = "https://github.com/lofantine/Rubik";

const buttonClass =
  "w-56 rounded-lg px-6 py-3 text-center font-medium transition-colors";

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-center gap-10 bg-neutral-900 text-neutral-100">
      <div className="flex flex-col items-center gap-4">
        {/* Face avant d'un cube résolu, utilisée comme logo */}
        <div className="grid grid-cols-3 gap-1 rounded-lg bg-black p-1.5">
          {Array.from({ length: 9 }, (_, i) => (
            <div key={i} className="size-7 rounded-sm bg-green-600" />
          ))}
        </div>
        <h1 className="text-5xl font-bold tracking-tight">Rubik</h1>
        <p className="text-neutral-400">Rubik's cube solver</p>
      </div>

      <nav className="flex flex-col items-center gap-3">
        <Link to="/working" className={`${buttonClass} bg-blue-600 hover:bg-blue-500`}>
          Start
        </Link>
        <a
          href={`${GITHUB_URL}#readme`}
          target="_blank"
          className={`${buttonClass} bg-neutral-800 hover:bg-neutral-700`}
        >
          Readme
        </a>
        <a
          href={GITHUB_URL}
          target="_blank"
          className={`${buttonClass} bg-neutral-800 hover:bg-neutral-700`}
        >
          GitHub
        </a>
      </nav>
    </main>
  );
}
