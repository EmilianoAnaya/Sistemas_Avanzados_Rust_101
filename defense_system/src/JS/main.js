const { invoke } = window.__TAURI__.core;

const Chart = window.Chart;

let currentChart = null;
let currentName = null;

let chartInterval = null;
let monitoring_flag = false;

window.addEventListener("DOMContentLoaded", () => {
  // Delegar eventos para navegación SPA
  document.body.addEventListener("click", (e) => {
    if (e.target.matches("[data-view]")) {
      const view = e.target.dataset.view;
      loadView(view);
    }
  });

  document.body.addEventListener("click", (e) => {
    if (e.target.matches(".metric-button")) {
      const label = e.target.textContent.trim().toLowerCase();
      switch (label) {
        case "cpu":
          show_cpu_stats("cpu");
          break;
        case "memory":
          show_memory_stats("memory");
          break;
        case "network":
          show_network_stats("network")  
          break;
      }
    }
  });

  fetch("Templates/header.html")
    .then(response => response.text())
    .then(data => {
      document.getElementById("main-header").innerHTML = data
    });

  loadView("home");
});


function loadView(viewName) {
  const main = document.getElementById("main-content");

  fetch(`Templates/${viewName}.html`)
    .then(res => res.text())
    .then(html => {
      main.innerHTML = html;
    })

    .catch(err => {
      main.innerHTML = "<p>Error al cargar la vista.</p>";
      console.error(err);
    });
}

function hide_current_chart(chartName){
  if (chartInterval) {
    clearInterval(chartInterval);
    chartInterval = null;
  }

  if (currentName !== chartName && currentChart) {
    if (currentChart.destroy) {
      currentChart.destroy();
    } else {
      Object.values(currentChart).forEach(chart => chart?.destroy && chart.destroy());
    }

    currentChart = null;
    currentName = chartName;
  }
}

function get_graph_container(){
  const graphs_container = document.getElementById("graphics-side")
  if (!graphs_container) return;
  return graphs_container
}

function show_cpu_stats(chartName) {
  hide_current_chart(chartName);
  
  const graphs_container = get_graph_container();
  graphs_container.innerHTML = `<canvas id="cpuChart" width="400" height="200"></canvas>`;
  
  const ctx = document.getElementById("cpuChart")?.getContext("2d");
  if (!ctx) return;
 
  const labels = [];
  const dataPoints = [];

  currentChart = new Chart(ctx, {
    type: 'line',
    data: {
      labels,
      datasets: [{
        label: 'Uso de CPU (%)',
        data: dataPoints,
        borderColor: 'rgba(75, 192, 192, 1)',
        borderWidth: 2,
        fill: true,
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

  chartInterval = setInterval(async () => {
    const usage = await invoke("get_cpu_usage");
    const now = new Date().toLocaleTimeString();

    labels.push(now);
    dataPoints.push(usage);

    if (labels.length > 30) {
      labels.shift();
      dataPoints.shift();
    }

    currentChart?.update?.();
  }, 1000);
}

function show_memory_stats(chartName) {
  hide_current_chart(chartName);

  const graphs_container = get_graph_container();
  graphs_container.innerHTML = `<canvas id="memoryChart" width="400" height="200"></canvas>`
  
  const ctx = document.getElementById("memoryChart")?.getContext("2d");
  if (!ctx) return;

  const labels = [];
  const dataPhysic = [];
  const dataSwap = [];
  const dataCache = [];

  currentChart = new Chart(ctx, {
    type: 'line',
    data: {
      labels,
      datasets: [
        {
          label: 'RAM Usada (GB)',
          data: dataPhysic,
          borderColor: 'rgba(255, 99, 132, 1)',
          borderWidth: 2,
          fill: true,
        },
        {
          label: 'Swap Usada (GB)',
          data: dataSwap,
          borderColor: 'rgba(54, 162, 235, 1)',
          borderWidth: 2,
          fill: true,
        },
        {
          label: 'Cache Libre (GB)',
          data: dataCache,
          borderColor: 'rgba(255, 206, 86, 1)',
          borderWidth: 2,
          fill: true,
        }
      ]
    },
    options: {
      animation: false,
      responsive: true,
      scales: {
        y: {
          min: 0
        }
      }
    }
  });

  chartInterval = setInterval(async () => {
    if (!currentChart) {
      clearInterval(chartInterval);
      chartInterval = null
      return;
    }

    const stats = await invoke("get_memory_stats");
    const now = new Date().toLocaleTimeString();

    labels.push(now);
    dataPhysic.push(stats.physic.toFixed(2));
    dataSwap.push(stats.swap.toFixed(2));
    dataCache.push(stats.cache.toFixed(2));

    if (labels.length > 30) {
      labels.shift();
      dataPhysic.shift();
      dataSwap.shift();
      dataCache.shift();
    }

    currentChart?.update?.();
  }, 1000);

}

function show_network_stats(chartName) {
  hide_current_chart(chartName);

  const graphs_container = get_graph_container();
  graphs_container.innerHTML = `
    <canvas id="rec-trans-chart" width="400" height="200"></canvas>
    <canvas id="active-chart" width="400" height="200"></canvas>
  `;

  const recTransCtx = document.getElementById("rec-trans-chart")?.getContext("2d");
  const activeCtx = document.getElementById("active-chart")?.getContext("2d");
  if (!recTransCtx || !activeCtx) return;

  const labels = [];
  const receivedData = [];
  const transmittedData = [];
  const activeConnections = [];

  const recTransChart = new Chart(recTransCtx, {
    type: 'line',
    data: {
      labels,
      datasets: [
        {
          label: 'Recibidos (Mbps)',
          data: receivedData,
          borderColor: 'rgba(75, 192, 192, 1)',
          borderWidth: 2,
          fill: false,
        },
        {
          label: 'Transmitidos (Mbps)',
          data: transmittedData,
          borderColor: 'rgba(255, 99, 132, 1)',
          borderWidth: 2,
          fill: false,
        }
      ]
    },
    options: {
      animation: false,
      responsive: true,
      scales: {
        y: {
          beginAtZero: true
        }
      }
    }
  });

  const activeChart = new Chart(activeCtx, {
    type: 'line',
    data: {
      labels,
      datasets: [
        {
          label: 'Conexiones activas',
          data: activeConnections,
          borderColor: 'rgba(153, 102, 255, 1)',
          borderWidth: 2,
          fill: false,
        }
      ]
    },
    options: {
      animation: false,
      responsive: true,
      scales: {
        y: {
          beginAtZero: true
        }
      }
    }
  });

  currentChart = { recTransChart, activeChart };

  chartInterval = setInterval(async () => {
    if (!currentChart || !currentChart.recTransChart || !currentChart.activeChart) {
      clearInterval(chartInterval);
      chartInterval = null;
      return;
    }

    const stats = await invoke("get_network_stats");
    const now = new Date().toLocaleTimeString();

    labels.push(now);
    receivedData.push(stats.received.toFixed(2));
    transmittedData.push(stats.transmitted.toFixed(2));
    activeConnections.push(stats.active);

    if (labels.length > 30) {
      labels.shift();
      receivedData.shift();
      transmittedData.shift();
      activeConnections.shift();
    }

    currentChart.recTransChart.update();
    currentChart.activeChart.update();
  }, 1000);
}