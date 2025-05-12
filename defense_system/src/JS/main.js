const { invoke } = window.__TAURI__.core;

const Chart = window.Chart;
// let greetInputEl;
// let greetMsgEl;

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//   greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
// }

window.addEventListener("DOMContentLoaded", () => {
  // Delegar eventos para navegación SPA
  document.body.addEventListener("click", (e) => {
    if (e.target.matches("[data-view]")) {
      const view = e.target.dataset.view;
      loadView(view);
    }
  });

  loadView("home");
});


function loadView(viewName) {
  const main = document.getElementById("main-content");

  // Para otras vistas
  fetch(`Templates/${viewName}.html`)
    .then(res => res.text())
    .then(html => {
      main.innerHTML = html;

      if (viewName === 'home'){
        setupCpuChart();
      }
    })
    .catch(err => {
      main.innerHTML = "<p>Error al cargar la vista.</p>";
      console.error(err);
    });
}

function setupCpuChart() {
  const ctx = document.getElementById("cpuChart")?.getContext("2d");
  if (!ctx) return;
// 
  const labels = [];
  const dataPoints = [];
// 
  const cpuChart = new Chart(ctx, {
    type: 'line',
    data: {
      labels,
      datasets: [{
        label: 'Uso de CPU (%)',
        data: dataPoints,
        borderColor: 'rgba(75, 192, 192, 1)',
        borderWidth: 2,
        fill: false,
      }]
    },
    options: {
      animation: false,
      responsive: true,
      scales: {
        y: {
          min: 0,
          max: 100
        }
      }
    }
  });
// 
  setInterval(async () => {
    const usage = await invoke("get_cpu_usage");
    const now = new Date().toLocaleTimeString();
// 
    labels.push(now);
    dataPoints.push(usage);
// 
    if (labels.length > 30) {
      labels.shift();
      dataPoints.shift();
    }
// 
    cpuChart.update();
  }, 1000);
}