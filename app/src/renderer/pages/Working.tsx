import { useEffect, useRef, useState } from "react";
import { useNavigate } from "react-router";
import * as THREE from 'three';

export default function Working() {
  const navigate = useNavigate();
  const mountRef = useRef(null);
  const [size] = useState(3);

  useEffect(() => {
    const scene = new THREE.Scene();
    scene.background = new THREE.Color(0x222222);
    const camera = new THREE.PerspectiveCamera( 75, window.innerWidth / window.innerHeight, 0.1, 1000 ); 
    const renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setSize(window.innerWidth, window.innerHeight);

    mountRef.current.appendChild(renderer.domElement);

    const root = new THREE.Group();
    scene.add(root);

    // Taille des cubelets et espacement
    const cubeSize = 0.95;
    const geometry = new THREE.BoxGeometry(cubeSize, cubeSize, cubeSize);
    
    const colors = {
      white:  new THREE.MeshBasicMaterial({ color: 0xffffff }),
      yellow: new THREE.MeshBasicMaterial({ color: 0xffff00 }),
      blue:   new THREE.MeshBasicMaterial({ color: 0x0000ff }),
      green:  new THREE.MeshBasicMaterial({ color: 0x00ff00 }),
      red:    new THREE.MeshBasicMaterial({ color: 0xff0000 }),
      orange: new THREE.MeshBasicMaterial({ color: 0xffa500 }),
      black:   new THREE.MeshBasicMaterial({ color: 0x000000 }),
    };

    const internalMaterial = colors.black;

    const root_groups : THREE.Group[] = [];

    // Génération du cube complet (3x3x3)
    for (let x = -1; x <= 1; x++) {
      for (let y = -1; y <= 1; y++) {
        for (let z = -1; z <= 1; z++) {
          
          // Détermination de la couleur de chaque face
          // +X: Red, -X: Orange, +Y: White, -Y: Yellow, +Z: Green, -Z: Blue
          const materials = [
            x === 1  ? colors.red    : internalMaterial, // +X
            x === -1 ? colors.orange  : internalMaterial, // -X
            y === 1  ? colors.white  : internalMaterial, // +Y
            y === -1 ? colors.yellow  : internalMaterial, // -Y
            z === 1  ? colors.green  : internalMaterial, // +Z
            z === -1 ? colors.blue    : internalMaterial, // -Z
          ];

          const cubelet_root = new THREE.Group();
          root.add(cubelet_root);

          root_groups.push(cubelet_root);
          
          const cubelet = new THREE.Mesh(geometry, materials);
          cubelet.position.set(x, y, z);
          cubelet_root.add(cubelet);
        }
      }
    }

    camera.position.set(4, 4, 6);
    camera.lookAt(0, 0, 0);

    const movement_speed = 0.01;

    const left_face = [
      0, 1, 2, 3 ,4 ,5 , 6, 7 , 8
    ]
    
    const animate = () => {
      requestAnimationFrame(animate);
      // root.rotation.x += 0.005;
			// root.rotation.y += 0.005;

      for (const i of left_face) {
        root_groups[i].rotation.x += movement_speed;
			}

			
      renderer.render(scene, camera);
    };
    animate();

    return () => {
      mountRef.current?.removeChild(renderer.domElement);
      renderer.dispose();
    }
  }, [size])

  return (
    <main>
      <h1 style={{ position: 'absolute', top: 20, left: 20, color: 'white', zIndex: 10, fontFamily: 'sans-serif' }}>
        Rubik's Cube - Full Model
      </h1>
      <div ref={mountRef}/>
    </main>
  );
}
