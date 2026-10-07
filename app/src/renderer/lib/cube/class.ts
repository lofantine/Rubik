import * as Three from 'three';
import { sleep } from '../utils/sleep';

const ANIMATION_TIME_MS = 500;
const ANIMATION_FRAMES = 100;

enum Face {
  Top,
  Bottom,
  Left,
  Right,
  Front,
  Back
}

type Move = { face: Face; prime: boolean };

// Axe et tranche de chaque face. sign = sens horaire vu depuis la face
// (une rotation positive est anti-horaire quand on regarde depuis le côté + de l'axe)
const FACE_ROTATIONS: Record<Face, { axis: 'x' | 'y' | 'z'; side: 'min' | 'max'; sign: 1 | -1 }> = {
  [Face.Right]:  { axis: 'x', side: 'max', sign: -1 },
  [Face.Left]:   { axis: 'x', side: 'min', sign: 1 },
  [Face.Top]:    { axis: 'y', side: 'max', sign: -1 },
  [Face.Bottom]: { axis: 'y', side: 'min', sign: 1 },
  [Face.Front]:  { axis: 'z', side: 'max', sign: -1 },
  [Face.Back]:   { axis: 'z', side: 'min', sign: 1 },
};

const COLORS = {
  white:  0xffffff,
  yellow: 0xffff00,
  blue:   0x0000ff,
  green:  0x00ff00,
  red:    0xff0000,
  orange: 0xffa500,
  black:   0x000000,
};

class Cube {
  public scene: Three.Scene;
  public camera: Three.PerspectiveCamera;
  public renderer: Three.WebGLRenderer;
  public size: number;
  public root: Three.Group;
  public cubelets: Three.Mesh[] = [];
  private materials: Map<Face, Three.MeshBasicMaterial> = new Map();
  private animationId: number | null;


  // status
  private status: 'animating' | 'idle';
  private queue: Move[];

  constructor(size = 3) {
    this.size = size;

    this.scene = new Three.Scene();
    this.camera = new Three.PerspectiveCamera(
      75,
      window.innerWidth / window.innerHeight,
      0.1,
      1000,
    );
    this.renderer = new Three.WebGLRenderer({ antialias: true });
    this.renderer.setSize(window.innerWidth, window.innerHeight);

    this.root = new Three.Group();
    this.scene.add(this.root);
    this.animationId = null;

    this.status = 'idle';
    this.queue = [];

    this.construct_cube();
  }

  public set_background_color(representation: Three.ColorRepresentation) {
    this.scene.background = new Three.Color(representation);
  }

  private construct_cube() {
    // Mapping des matériaux selon les faces
    this.materials.set(Face.Top, new Three.MeshBasicMaterial({ color: COLORS.white }));
    this.materials.set(Face.Bottom, new Three.MeshBasicMaterial({ color: COLORS.yellow }));
    this.materials.set(Face.Left, new Three.MeshBasicMaterial({ color: COLORS.orange }));
    this.materials.set(Face.Right, new Three.MeshBasicMaterial({ color: COLORS.red }));
    this.materials.set(Face.Front, new Three.MeshBasicMaterial({ color: COLORS.green }));
    this.materials.set(Face.Back, new Three.MeshBasicMaterial({ color: COLORS.blue }));

    const internalMaterial = new Three.MeshBasicMaterial({ color: COLORS.black });
    const cubeSize = 0.95;
    const geometry = new Three.BoxGeometry(cubeSize, cubeSize, cubeSize);

    const offset = (this.size - 1) / 2;

    for (let x = 0; x < this.size; x++) {
      for (let y = 0; y < this.size; y++) {
        for (let z = 0; z < this.size; z++) {

          const materials = [
            x === this.size - 1 ? this.materials.get(Face.Right)! : internalMaterial,
            x === 0            ? this.materials.get(Face.Left)!  : internalMaterial,
            y === this.size - 1 ? this.materials.get(Face.Top)!   : internalMaterial,
            y === 0            ? this.materials.get(Face.Bottom)!: internalMaterial,
            z === this.size - 1 ? this.materials.get(Face.Front)! : internalMaterial,
            z === 0            ? this.materials.get(Face.Back)!  : internalMaterial,
          ];

          // Le cubelet est directement enfant de root : sa position est mise à jour
          // à chaque rotation, ce qui permet de savoir dans quelle tranche il se trouve
          const cubelet = new Three.Mesh(geometry, materials);

          // Centrage du cube
          cubelet.position.set(x - offset, y - offset, z - offset);

          this.root.add(cubelet);
          this.cubelets.push(cubelet);
        }
      }
    }

    this.set_background_color(0x222222);
    this.camera.position.set(4, 4, 6);
    this.camera.lookAt(0, 0, 0);
  }

  public rotateFace(axis: 'x' | 'y' | 'z', layer: number, angle: number) {
    const offset = (this.size - 1) / 2;
    const target = layer - offset;

    const axisVector = new Three.Vector3(
      axis === 'x' ? 1 : 0,
      axis === 'y' ? 1 : 0,
      axis === 'z' ? 1 : 0,
    );
    const quaternion = new Three.Quaternion().setFromAxisAngle(axisVector, angle);

    this.cubelets.forEach((cubelet) => {
      // Sélection sur la position actuelle : la coordonnée sur l'axe de rotation
      // ne change pas pendant le mouvement, donc la sélection reste stable même
      // en pleine animation. La tolérance absorbe les erreurs d'arrondi flottant.
      if (Math.abs(cubelet.position[axis] - target) > 0.1) return;

      // Orbite autour du centre du cube (position) + rotation sur lui-même (orientation)
      cubelet.position.applyQuaternion(quaternion);
      cubelet.quaternion.premultiply(quaternion);
    });
  }

  // Ajoute un mouvement à la queue et lance son traitement si rien n'est en cours.
  // prime = sens anti-horaire (notation R', U'...)
  public rotate(face: Face, prime = false) {
    this.queue.push({ face, prime });
    if (this.status === 'idle') {
      this.handle_queue();
    }
  }

  private async handle_queue() {
    this.status = 'animating';
    while (this.queue.length > 0) {
      await this.animate_move(this.queue.shift()!);
    }
    this.status = 'idle';
  }

  private async animate_move({ face, prime }: Move) {
    const { axis, side, sign } = FACE_ROTATIONS[face];
    const layer = side === 'max' ? this.size - 1 : 0;
    const direction = prime ? -sign : sign;

    let angle = 0;
    const step = 90 / ANIMATION_FRAMES;
    while (angle < 90) {
      await sleep(ANIMATION_TIME_MS / ANIMATION_FRAMES);

      const calc_angle = (angle + step) >= 90 ? 90 - angle : step;
      this.rotateFace(axis, layer, Three.MathUtils.degToRad(calc_angle) * direction);
      angle += calc_angle;
    }
  }

  public animate = () => {
    this.animationId = requestAnimationFrame(this.animate);

    // Animation de test : rotation automatique du cube entier
    // this.root.rotation.x += 0.005;
    // this.root.rotation.y += 0.005;

    // this.rotateFace('z', 1, 0.005);

    this.renderer.render(this.scene, this.camera);
  };

  public drop() {
    cancelAnimationFrame(this.animationId!);
    this.renderer.dispose();
  }
}

export { Cube, Face };
export type { Move };
