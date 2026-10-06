import { useEffect, useRef, useState } from "react";
import { useNavigate } from "react-router";
import { Cube } from '../lib/cube/class';

export default function Working() {
  const navigate = useNavigate();
  const mountRef = useRef(null);
  const [size] = useState(3);

  useEffect(() => {
    // Utilisation de la classe Cube qui gère sa propre scene, camera et renderer
    const cube = new Cube(size);

    if (mountRef.current) {
      mountRef.current.appendChild(cube.renderer.domElement);
    }

    // On lance la boucle d'animation définie dans la classe
    cube.animate();

    return () => {
      if (mountRef.current) {
        mountRef.current.removeChild(cube.renderer.domElement);
      }
      cube.drop();
    }
  }, [size])

  return (
    <main>
      <h1 style={{ position: 'absolute', top: 20, left: 20, color: 'white', zIndex: 10, fontFamily: 'sans-serif' }}>
        Rubik's Cube - Class Implementation
      </h1>
      <div ref={mountRef}/>
    </main>
  );
}
