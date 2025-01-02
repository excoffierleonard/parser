// Get DOM elements
const uploadArea = document.getElementById('uploadArea');
const fileInput = document.getElementById('fileInput');
const uploadButton = document.getElementById('uploadButton');
const fileName = document.getElementById('fileName');
const spinner = document.getElementById('spinner');
const responseArea = document.getElementById('responseArea');
const responseText = document.getElementById('responseText');
const copyButton = document.getElementById('copyButton');

// Handle drag and drop events
['dragenter', 'dragover', 'dragleave', 'drop'].forEach(eventName => {
    uploadArea.addEventListener(eventName, preventDefaults);
});

function preventDefaults(e) {
    e.preventDefault();
    e.stopPropagation();
}

// Add visual feedback for drag and drop
['dragenter', 'dragover'].forEach(eventName => {
    uploadArea.addEventListener(eventName, () => {
        uploadArea.classList.add('drag-over');
    });
});

['dragleave', 'drop'].forEach(eventName => {
    uploadArea.addEventListener(eventName, () => {
        uploadArea.classList.remove('drag-over');
    });
});

// Handle file drop
uploadArea.addEventListener('drop', handleDrop);

function handleDrop(e) {
    const dt = e.dataTransfer;
    const file = dt.files[0];
    handleFile(file);
}

// Handle file selection via button
uploadButton.addEventListener('click', () => fileInput.click());
fileInput.addEventListener('change', (e) => {
    handleFile(e.target.files[0]);
});

// Process the selected file
function handleFile(file) {
    if (!file) return;

    // Display file name
    fileName.textContent = `Selected file: ${file.name}`;

    // Create FormData and append file
    const formData = new FormData();
    formData.append('file', file);

    // Show loading spinner
    spinner.style.display = 'block';
    responseArea.style.display = 'none';

    // Send POST request
    fetch('/parse', {
        method: 'POST',
        body: formData
    })
        .then(response => response.json())
        .then(data => {
            // Handle successful response
            spinner.style.display = 'none';
            responseArea.style.display = 'block';

            if (data.text) {
                // Success response
                responseArea.className = 'response-area success';
                responseText.textContent = data.text;
            } else if (data.message) {
                // Error response
                responseArea.className = 'response-area error';
                responseText.textContent = `Error: ${data.message}`;
            }
        })
        .catch(error => {
            // Handle network or other errors
            spinner.style.display = 'none';
            responseArea.style.display = 'block';
            responseArea.className = 'response-area error';
            responseArea.textContent = `Error: ${error.message}`;
        });
}

// Handles store to clipboard
copyButton.addEventListener('click', () => {
    navigator.clipboard.writeText(responseText.textContent);
});