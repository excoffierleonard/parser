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

// Process the selected files
function handleFiles(files) {
    if (!files || files.length === 0) return;

    // Display file names
    fileName.textContent = `Selected files: ${Array.from(files).map(f => f.name).join(', ')}`;

    // Create FormData and append files
    const formData = new FormData();
    Array.from(files).forEach(file => {
        formData.append('file', file);
    });

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

            if (data.texts) {
                // Success response
                responseArea.className = 'response-area success';
                responseText.innerHTML = ''; // Clear previous content

                // Create a box for each text
                data.texts.forEach((text, index) => {
                    const textBox = document.createElement('div');
                    textBox.className = 'text-box';

                    const copyButton = document.createElement('button');
                    copyButton.className = 'copy-button';
                    copyButton.innerHTML = '📋';
                    copyButton.onclick = () => navigator.clipboard.writeText(text);

                    const textContent = document.createElement('div');
                    textContent.className = 'text-content';
                    textContent.textContent = text;

                    textBox.appendChild(copyButton);
                    textBox.appendChild(textContent);
                    responseText.appendChild(textBox);
                });
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
            responseText.textContent = `Error: ${error.message}`;
        });
}

// Update the file drop handler
function handleDrop(e) {
    const dt = e.dataTransfer;
    handleFiles(dt.files);
}

// Update the file selection handler
fileInput.addEventListener('change', (e) => {
    handleFiles(e.target.files);
});

// Handles store to clipboard
copyButton.addEventListener('click', () => {
    navigator.clipboard.writeText(responseText.textContent);
});