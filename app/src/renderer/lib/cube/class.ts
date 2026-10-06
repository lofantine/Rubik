import * as Three from 'three';

enum Face {
  Top,
  Bottom,
  Left,
  Right,
  Front,
  Back
}

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
  public cubelets: Three.Group[] = [];
  private materials: Map<Face, Three.MeshBasicMaterial> = new Map();

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
            x === this.size - 1 ? this.materials.get(Face.Right) : internalMaterial,
            x === 0            ? this.materials.get(Face.Left)  : internalMaterial,
            y === this.size - 1 ? this.materials.get(Face.Top)   : internalMaterial,
            y === 0            ? this.materials.get(Face.Bottom): internalMaterial,
            z === this.size - 1 ? this.materials.get(Face.Front) : internalMaterial,
            z === 0            ? this.materials.get(Face.Back)  : internalMaterial,
          ];

          const cubeletRoot = new Three.Group();
          const cubelet = new Three.Mesh(geometry, materials);
          
          // Centrage du cube
          cubelet.position.set(x - offset, y - offset, z - offset);
          
          cubeletRoot.add(cubelet);
          this.root.add(cubeletRoot);
          this.cubelets.push(cubeletRoot);
        }
      }
    }

    this.set_background_color(0x222222);
    this.camera.position.set(4, 4, 6);
    this.camera.lookAt(0, 0, 0);
  }

  public rotateFace(axis: 'x' | 'y' | 'z', layer: number, angle: number) {
    const offset = (this.size - 1) / 2;
    this.cubelets.forEach((cubelet) => {
      const mesh = cubelet.children[0] as Three.Mesh;
      const pos = mesh.position;

      let shouldRotate = false;
      // On compare avec la position relative au centre
      if (axis === 'x' && pos.x === layer - offset) shouldRotate = true;
      if (axis === 'y' && pos.y === layer - offset) shouldRotate = true;
      if (axis === 'z' && pos.z === layer - offset) shouldRotate = true;

      if (shouldRotate) {
        if (axis === 'x') cubelet.rotation.x += angle;
        if (axis === 'y') cubelet.rotation.y += angle;
        if (axis === 'z') cubelet.rotation.z += angle;
      }
    });
  }

  public animate = () => {
    requestAnimationFrame(this.animate);
    
    // Animation de test : rotation automatique du cube entier
    // this.root.rotation.x += 0.005;
    // this.root.rotation.y += 0.005;
    
    this.rotateFace('z', 1, 0.005);
    
    this.renderer.render(this.scene, this.camera);
  };

  public drop() {
    this.renderer.dispose();
  }
}

export { Cube, Face };
