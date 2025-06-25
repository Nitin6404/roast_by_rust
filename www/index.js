import init, { get_roast } from './pkg/roast_wasm.js';

async function roast() {
  await init(); // Load WASM
  const roastButton = document.getElementById('roastButton');
  const resultDiv = document.getElementById('result');

  roastButton.addEventListener('click', () => {
    const name = document.getElementById('name').value.trim() || "you";
    const roast = get_roast(name);
    resultDiv.textContent = roast;
    resultDiv.style.animation = 'none';
    void resultDiv.offsetWidth; // restart animation
    resultDiv.style.animation = null;
  });
}

roast();
