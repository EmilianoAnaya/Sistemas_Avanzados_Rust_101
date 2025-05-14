const { invoke } = window.__TAURI__.core;

const Chart = window.Chart;

let currentChart = null;
let currentName = null;

let chartInterval = null;

let monitor_recolection = null;

let labels = [];

let cpuUsage = [];

let dataPhysic = [];
let dataSwap = [];
let dataCache = [];

let receivedData = [];
let transmittedData = [];
let activeConnections = [];

let diskRead = [];
let diskWrite = [];
let operationE = [];
let operationS = [];

let processesData = [];
let coresUsage = [];

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
  graphs_container.innerHTML = `
    <h1>CPU</h1>
    <canvas id="cpuChart" width="400" height="200"></canvas>
    <table id="cpuTable" border="1">
      <thead>
        <tr>
          <th>Core</th>
          <th>Uso (%)</th>
        </tr>
      </thead>
      <tbody></tbody>
    </table>
    `
  
  const ctx = document.getElementById("cpuChart")?.getContext("2d");
  if (!ctx) return;
 
  currentChart = new Chart(ctx, {
    type: 'line',
    data: {
      labels,
      datasets: [{
        label: 'Uso de CPU (%)',
        data: cpuUsage,
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
    if (!currentChart) {
      clearInterval(chartInterval);
      chartInterval = null
      return;
    }

    currentChart?.update?.();

    // Actualizar tabla
    const tbody = document.querySelector("#cpuTable tbody");
    tbody.innerHTML = ""; // Limpiar tabla anterior

    coresUsage.forEach((value, index) => {
      const row = document.createElement("tr");

      const coreCell = document.createElement("td");
      coreCell.textContent = `Core ${index}`;

      const usageCell = document.createElement("td");
      usageCell.textContent = value.toFixed(2) + " %";

      row.appendChild(coreCell);
      row.appendChild(usageCell);
      tbody.appendChild(row);
    });

  }, 1000);
}

function show_memory_stats(chartName) {
  hide_current_chart(chartName);

  const graphs_container = get_graph_container();
  graphs_container.innerHTML = `
    <h1>Memory</h1>
    <canvas id="memoryChart" width="400" height="200"></canvas>
    `
  
  const ctx = document.getElementById("memoryChart")?.getContext("2d");
  if (!ctx) return;

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

    currentChart?.update?.();
  }, 1000);

}

function show_network_stats(chartName) {
  hide_current_chart(chartName);

  const graphs_container = get_graph_container();
  graphs_container.innerHTML = `
    <h1>Network</h1>
    <canvas id="rec-trans-chart" width="400" height="200"></canvas>
    <canvas id="active-chart" width="400" height="200"></canvas>
  `;

  const recTransCtx = document.getElementById("rec-trans-chart")?.getContext("2d");
  const activeCtx = document.getElementById("active-chart")?.getContext("2d");
  if (!recTransCtx || !activeCtx) return;

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

  const ReWrChart = new Chart(activeCtx, {
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

  currentChart = { recTransChart, ReWrChart };

  chartInterval = setInterval(async () => {
    if (!currentChart || !currentChart.recTransChart || !currentChart.ReWrChart) {
      clearInterval(chartInterval);
      chartInterval = null;
      return;
    }

    currentChart.recTransChart.update();
    currentChart.ReWrChart.update();
  }, 1000);
}

function show_disks_stats(chartName) {
  hide_current_chart(chartName)

  const graphs_container = get_graph_container();
  graphs_container.innerHTML = `
    <h1>Disks</h1>
    <canvas id="op-e-s" width="400" height="200"></canvas>
    <canvas id="read-write" width="400" height="200"></canvas>
  `;

  const OpESCtx = document.getElementById("op-e-s")?.getContext("2d");
  const ReWrCtx = document.getElementById("read-write")?.getContext("2d");
  if (!OpESCtx || !ReWrCtx) return;

  const OpESChart = new Chart(OpESCtx, {
    type: 'line',
    data: {
      labels,
      datasets: [
        {
          label: 'Escritura (IOPS)',
          data: operationE,
          borderColor: 'rgba(75, 192, 192, 1)',
          borderWidth: 2,
          fill: false,
        },
        {
          label: 'Lectura (IOPS)',
          data: operationS,
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

  const ReWrChart = new Chart(ReWrCtx, {
    type: 'line',
    data: {
      labels,
      datasets: [
        {
          label: 'Lectura (Mbps)',
          data: diskRead,
          borderColor: 'rgb(59, 190, 59)',
          borderWidth: 2,
          fill: false,
        },
        {
          label: 'Escritura (Mbps)',
          data: diskWrite,
          borderColor: 'rgb(171, 82, 206)',
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

  currentChart = { OpESChart, ReWrChart };

  chartInterval = setInterval(async () => {
    if (!currentChart || !currentChart.OpESChart || !currentChart.ReWrChart) {
      clearInterval(chartInterval);
      chartInterval = null;
      return;
    }

    currentChart.OpESChart.update();
    currentChart.ReWrChart.update();
  }, 1000);
}

function show_processes_stats(chartName) {
  hide_current_chart(chartName);

  const graphs_container = get_graph_container();
  graphs_container.innerHTML = `
    <h1>Processs</h1>
    <canvas id="processes" width="400" height="200"></canvas>
  `;

  const ProcCtx = document.getElementById("processes")?.getContext("2d");
  if (!ProcCtx) return;

  // Inicializar el gráfico
  currentChart = new Chart(ProcCtx, {
    type: 'bar',
    data: {
      labels: processesData.map(p => p.name), // Nombres iniciales
      datasets: [{
        label: 'Uso de CPU (%)',
        data: processesData.map(p => p.cpu.toFixed(4)), // CPU inicial
        backgroundColor: 'rgba(75, 192, 192, 0.6)',
        borderColor: 'rgba(75, 192, 192, 1)',
        borderWidth: 1
      }]
    },
    options: {
      responsive: true,
      scales: {
        y: {
          beginAtZero: true,
          title: {
            display: true,
            text: '% de uso'
          }
        },
        x: {
          ticks: {
            autoSkip: false
          }
        }
      }
    }
  });

  // Actualizar los datos del gráfico en cada intervalo
  chartInterval = setInterval(async () => {
    if (!currentChart) {
      clearInterval(chartInterval);
      chartInterval = null;
      return;
    }

    // Actualizar los datos del gráfico con los valores más recientes de processesData
    currentChart.data.labels = processesData.map(p => p.name);
    currentChart.data.datasets[0].data = processesData.map(p => p.cpu.toFixed(4));
    currentChart.update(); // Redibujar el gráfico con los nuevos datos
  }, 1000);
}

async function fetch_metrics_data() {
  try {
      // const data = await invoke('start_monitoring', {
      //   threshold: 80.0, // Puedes enviar cualquier parámetro
      //   interval: 1000
      // });

      const data = await invoke('start_monitoring');
      
      const now = new Date().toLocaleTimeString();
      let cpu_usage = data.CPU;
      let memory = data.Memory;
      let network = data.Network;
      let disk = data.Disk;
      processesData = data.Proccess
      coresUsage = cpu_usage.per_core

      labels.push(now)
      
      cpuUsage.push(cpu_usage.global)

      dataPhysic.push(memory.physic.toFixed(2));
      dataSwap.push(memory.swap.toFixed(2));
      dataCache.push(memory.cache.toFixed(2));

      receivedData.push(network.received.toFixed(2));
      transmittedData.push(network.transmitted.toFixed(2));
      activeConnections.push(network.active);

      diskRead.push(disk.read_mbps);
      diskWrite.push(disk.write_mbps);
      operationE.push(disk.iops_write);
      operationS.push(disk.iops_read);

      if (labels.length > 30) {
        labels.shift();
        cpuUsage.shift();
        dataPhysic.shift();
        dataSwap.shift();
        dataCache.shift();
        receivedData.shift();
        transmittedData.shift();
        activeConnections.shift();
        diskRead.shift();
        diskWrite.shift();
        operationE.shift();
        operationS.shift();
      }

    } catch (error) {
      console.error("Error al obtener datos:", error);
    }
}

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
          show_cpu_stats(label);
          break;

        case "memory":
          show_memory_stats(label);
          break;

        case "network":
          show_network_stats(label)  
          break;

        case "disks":
          show_disks_stats(label)
          break
        
        case "processes":
          show_processes_stats(label)
          break
      }
    }
  });

  document.body.addEventListener("click", (e) => {
    if (e.target.matches("#trigger-on")) {
      if (monitor_recolection === null) {
        fetch_metrics_data();
        monitor_recolection = setInterval(fetch_metrics_data, 1000);
        
      }
    }
  });

  document.body.addEventListener("click", (e) => {
    if (e.target.matches("#trigger-off")) {
      if (monitor_recolection != null) {
        clearInterval(monitor_recolection);
        monitor_recolection = null;

      }
    }
  });

  fetch("Templates/header.html")
    .then(response => response.text())
    .then(data => {
      document.getElementById("main-header").innerHTML = data
    });

  loadView("umbrals");

});