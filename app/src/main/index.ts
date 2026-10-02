import { BrowserWindow, app, shell } from 'electron';
import path from 'node:path';

let mainWindow: BrowserWindow | null = null;

const isExternal = (url: string) => url.startsWith('https://') || url.startsWith('http://');

const launchWin = () => {
  mainWindow = new BrowserWindow({
    width: 800,
    height: 600,
  })
  mainWindow.setMenu(null);

  // Les liens externes (target="_blank") s'ouvrent dans le navigateur par défaut
  mainWindow.webContents.setWindowOpenHandler(({ url }) => {
    if (isExternal(url)) {
      shell.openExternal(url);
    }
    return { action: 'deny' };
  });

  // Empêche la fenêtre de naviguer vers un site externe
  mainWindow.webContents.on('will-navigate', (event, url) => {
    if (isExternal(url)) {
      event.preventDefault();
      shell.openExternal(url);
    }
  });

  mainWindow.loadFile(path.join(import.meta.dirname, 'renderer', 'index.html'));
}

app.whenReady().then(() => {
  launchWin();
})

app.on('window-all-closed', () => {
  app.quit();
})
