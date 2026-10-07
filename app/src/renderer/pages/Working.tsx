import { useCallback, useEffect, useRef, useState } from "react";
import { useNavigate } from "react-router";
import { Cube, Face } from '../lib/cube/class';

// Notation standard du Rubik's cube
const CONTROLS: { label: string; face: Face }[] = [
  { label: 'U', face: Face.Top },
  { label: 'D', face: Face.Bottom },
  { label: 'L', face: Face.Left },
  { label: 'R', face: Face.Right },
  { label: 'F', face: Face.Front },
  { label: 'B', face: Face.Back },
];

export default function Working() {
  const navigate = useNavigate();
  const mountRef = useRef<HTMLDivElement>(null);
  const [size] = useState(3);

  const cubeRef = useRef<Cube | null>(null);

  const rotate = useCallback((face: Face, prime: boolean) => {
    cubeRef.current?.rotate(face, prime);
  }, [])

  useEffect(() => {
    // Utilisation de la classe Cube qui gère sa propre scene, camera et renderer
    const cube = new Cube(size);
    cubeRef.current = cube;

    if (mountRef.current) {
      mountRef.current.appendChild(cube.renderer.domElement);
    }

    // On lance la boucle d'animation définie dans la classe
    cube.animate();

    return () => {
      cube.renderer.domElement.remove();
      cube.drop();
      cubeRef.current = null;
    }
  }, [size])

  // Clavier : U D L R F B, Shift pour le sens anti-horaire
  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.repeat) return;
      const control = CONTROLS.find((c) => c.label === event.key.toUpperCase());
      if (control) {
        rotate(control.face, event.shiftKey);
      }
    };

    window.addEventListener('keydown', onKeyDown);
    return () => window.removeEventListener('keydown', onKeyDown);
  }, [rotate])

  return (
    <main>
      <h1 style={{ position: 'absolute', top: 20, left: 20, color: 'white', zIndex: 10, fontFamily: 'sans-serif' }}>
        Rubik's Cube - Class Implementation
      </h1>

      <div className="absolute bottom-5 left-1/2 z-10 flex -translate-x-1/2 flex-col gap-2">
        {[false, true].map((prime) => (
          <div key={String(prime)} className="flex gap-2">
            {CONTROLS.map(({ label, face }) => (
              <button
                key={label}
                onClick={() => rotate(face, prime)}
                className="w-12 rounded bg-neutral-700 py-2 font-mono text-white hover:bg-neutral-600 active:bg-neutral-500"
              >
                {label}{prime ? "'" : ''}
              </button>
            ))}
          </div>
        ))}
        <p className="text-center text-xs text-neutral-400">
          Clavier : U D L R F B — Shift pour le sens inverse
        </p>
      </div>

      <div ref={mountRef}/>
    </main>
  );
}
