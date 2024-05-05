import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { open, message } from '@tauri-apps/api/dialog';
import { downloadDir } from '@tauri-apps/api/path';

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#import-btn")?.addEventListener("click", async () => {
    open({
      multiple: true,
      defaultPath: await downloadDir(),
      title: "Select files to import",
    }).then((files) => {
      if (files) {
        selectedFiles = [...selectedFiles, ...files];
        renderFiles();
      }
    });
  });
  document.querySelectorAll("button.help").forEach((btn) => {
    btn.addEventListener("click", async (e) => {
      // get data-message attribute and show it as a message
      const message = (e.target as HTMLElement).getAttribute("data-message");
      if (message) {
        await help(message);
      }
    });
  });
  document.querySelector("#to-processing-btn")?.addEventListener("click", () => {
    const processingSection = document.querySelector("#processing");
    const homeSection = document.querySelector("#home");
    if (processingSection && homeSection) {
      homeSection.classList.add("hidden");
      processingSection.classList.remove("hidden");
    }
  });
  document.querySelector("#to-import-btn")?.addEventListener("click", () => {
    const processingSection = document.querySelector("#processing");
    const homeSection = document.querySelector("#home");
    if (processingSection && homeSection) {
      homeSection.classList.remove("hidden");
      processingSection.classList.add("hidden");
    }
  });
  document.querySelector("#to-export-btn")?.addEventListener("click", async () => {
    const exportSection = document.querySelector("#export");
    const processingSection = document.querySelector("#processing");
    if (exportSection && processingSection) {
      processingSection.classList.add("hidden");
      exportSection.classList.remove("hidden");
      const filePath = await open({
        defaultPath: await downloadDir(),
        directory: true,
        title: "Select export location",
      });
      const preferred = document.querySelector("#perfered-format") as HTMLSelectElement;
      const create_in_folder = document.querySelector("#export-in-file") as HTMLInputElement;
      // get the processing result
      const result = await invoke("process_files", {
        files: selectedFiles, 
        destination: filePath, 
        preferred: preferred.value,
        create_in_folder: create_in_folder.checked,
        createInFolder: create_in_folder.checked, 
      });
      console.log(result);
      if (result) {
        const exportResult = document.querySelector("#export-result");
        if (exportResult) {
          // add p tags with the result
          const p = document.createElement("p");
          p.textContent = "Successfully processed the files";
          exportResult.replaceChildren(p);
          // open export location
          const openBtn = document.createElement("button");
          openBtn.textContent = "Open export location";
          openBtn.addEventListener("click", async () => {
            // open file location in file explorer
            await invoke("open_file_location", { filePath: filePath, file_path: filePath });
          });
          exportResult.appendChild(openBtn);
          // add a button to go back to the home screen
          const backBtn = document.createElement("button");
          backBtn.id = "back-to-home-btn";
          backBtn.textContent = "Back";
          backBtn.title = "Go back to the home screen";
          backBtn.addEventListener("click", () => {
            const homeSection = document.querySelector("#home");
            if (homeSection) {
              exportSection.classList.add("hidden");
              homeSection.classList.remove("hidden");
            }
          });
          exportResult.appendChild(backBtn);
        }
      }
    }
  });
});

var selectedFiles: Array<string> = [];

function renderFiles() {
  selectedFiles = [...new Set(selectedFiles)];
  const filesList = document.querySelector("#files-list");
  if (filesList) {
    filesList.innerHTML = "";
    selectedFiles.forEach((file) => {
      const li = document.createElement("li");
      li.classList.add("file");
      li.textContent = file;
      // add remove button
      const removeBtn = document.createElement("button");
      removeBtn.textContent = "❌";
      removeBtn.title = "Remove file";
      removeBtn.addEventListener("click", () => {
        selectedFiles = selectedFiles.filter((f) => f !== file);
        renderFiles();
      });
      li.appendChild(removeBtn);
      filesList.appendChild(li);
    });
    // set #to-processing-btn to enabled if there are files
    const toProcessingBtn = document.querySelector("#to-processing-btn");
    if (toProcessingBtn && selectedFiles.length > 0) {
      toProcessingBtn.removeAttribute("disabled");
    } else {
      toProcessingBtn?.setAttribute("disabled", "true");
    }
  }
  console.log(selectedFiles);
}


listen('tauri://file-drop', (event: any) => {
  selectedFiles = [...selectedFiles, ...event.payload];
  renderFiles();
  const doc = document.querySelector("html");
  if (doc) {
    doc.classList.remove("file-drag-hover");
  }
});

listen('tauri://file-drop-hover', () => {
  console.log("File drop hover");
  const doc = document.querySelector("html");
  if (doc) {
    doc.classList.add("file-drag-hover");
  }
});

listen('tauri://file-drop-cancelled', () => {
  console.log("File drop cancelled");
  const doc = document.querySelector("html");
  if (doc) {
    doc.classList.remove("file-drag-hover");
  }
});

async function help(text: string) {
  await message(
    text,
    {
      title: "Help",
      type: "info",
    }
  )
}