/*
* core.js
*
* contains the totality of the default runtime
* 
* contains loading screen status message callbacks
*
* checks cache for decoded modules
* loads brotli decoder
* decompresses the main wasm module
* initialize wasm
* 
* this works with auto start modules, it doesn't call anything in the module
* you will have to provide a start javascript yourself and include your glue
*
* the use of immediately invoked function expressions IIFE, creates a cleaner
* global namespace
*/

/*
* app
* orchestrates the wasm loading process with detailed status updates
*/

// we need to add a piece of code that makes sure the wasm loads
// a promise that the wasm has been loaded
// just do;
// await window.wasmReady;
window.wasmReady = new Promise((resolve) => {
    window.__resolveWasmReady = resolve;
});

// run app when the page is loaded
window.addEventListener('DOMContentLoaded', runCore);

async function runCore() {
    // loading screen with progress tracking
    const loadingScreen = window.createLoadingScreen();
    console.log("Starting WASM application...");
    
    try {
        // Pass the loading screen to setupWasm for status updates
        await window.setupWasm(loadingScreen);
        // Resolve the promise
        window.__resolveWasmReady(wasm_bindgen);

    } catch (error) {
        console.error("Fatal error starting WASM application:", error);
        loadingScreen.updateText("Error loading application. Please refresh the page.", 'error');
        return; // Don't hide the loading screen on error
    }
    
    // hide loading screen once WASM is loaded
    loadingScreen.updateText("Application ready!", 'success');
    /*
    setTimeout(() => {
        loadingScreen.hide();
    }, 500); // Short delay to show "ready" message
    */
    // instant or delayed?
    loadingScreen.hide();
}

/*
* decoder
* decodes the embedded application with status updates
*/
(async () => {
    // Main setup function exposed globally
    window.setupWasm = async function(loadingScreen) {
        /*
        const updateStatus = loadingScreen ? 
            (text) => loadingScreen.updateText(text) : 
            (text) => console.log(`Status: ${text}`);
        */
        // Create a status callback function if loadingScreen is provided
        const updateStatus = (text) => {
            console.log(`${text}`);
            loadingScreen?.updateText(text);
        }
        
        try {
            console.log("Setting up WASM application...");
            updateStatus("Initializing...");
            
            const db = await openDb(updateStatus);
            await loadDecoder(db, updateStatus);
            await loadApp(db, updateStatus);
            
            console.log("WASM module initialized successfully!");
        } catch (e) {
            console.error("WASM setup error:", e);
            throw e;
        }
    };

    // Module Loading -------------------------------------------------------- /

    // specific callers for the two modules
    async function loadDecoder(db, statusCallback) {
        await loadModule(
            db, 
            {
                elementId: 'bin-wasm-decoder',
                cachePrefix: 'wasm-decoder',
                name: 'decoder module',
                shouldDecompress: false,
                initFunction: wasm_decoder
            },
            statusCallback
        );
    }
    
    async function loadApp(db, statusCallback) {
        await loadModule(
            db, 
            {
                elementId: 'bin-wasm-app',
                cachePrefix: 'wasm-app',
                name: 'main application',
                shouldDecompress: true,
                initFunction: wasm_bindgen
            },
            statusCallback
        );
    }

    // check WASM binary header for validity
    function checkMagicBytes(bytes) {
        const magic = Array.from(bytes.slice(0, 4));
        const p = magic
            .map(b => "0x" + b.toString(16).padStart(2, '0'))
            .join(' ');
        console.log("WASM binary magic bytes:", p);
        if (bytes.length < 4  || 
            bytes[0] !== 0x00 || 
            bytes[1] !== 0x61 || 
            bytes[2] !== 0x73 || 
            bytes[3] !== 0x6D) {
            throw new Error("Invalid WASM binary (wrong header)");
        }
    }
    
    // convert base64 to ArrayBuffer u8 bytes
    function b64ToBytes(base64) {
        base64 = base64.replace(/\s/g, '');
        const binaryString = atob(base64);
        const bytes = new Uint8Array(binaryString.length);
        for (let i = 0; i < binaryString.length; i++) {
            bytes[i] = binaryString.charCodeAt(i);
        }
        return bytes;
    }

    async function loadFromCache(db, cacheKey, statusCallback) {
            statusCallback?.("Checking cache...");
            const cachedBytes = await getAssetFromCache(db, cacheKey);
            if (cachedBytes) {
                statusCallback?.("Loaded from cache...");
                return cachedBytes;
            }
            return null;
    }

    async function decodeBinary(base64Data, shouldDecompress, statusCallback) {
        statusCallback?.("Decoding module...");

        let wasmBytes = b64ToBytes(base64Data);

        if (shouldDecompress) {
            statusCallback?.("Decompressing module...");
            wasmBytes = await wasm_decoder.decompress(wasmBytes);
        }

        checkMagicBytes(wasmBytes);
        return wasmBytes;
    }


    // main function that handles:
    // find element
    // get hash
    // check cache
    // if yes get from cache
    // else get innerhtml from element
    async function loadModule(db, config, statusCallback) {
        let wasmBytes;
        statusCallback?.(`Loading ${config.name}...`);
        
        // get from dom
        const element = document.getElementById(config.elementId);
        const hash = element.getAttribute('hash');

        // if no hash, get from inner
        // if yes hash, check if cached
        if (!hash) {
            const data = element.innerHTML.trim();
            wasmBytes = await decodeBinary(
                data, config.shouldDecompress, statusCallback);
        } else {
            console.log(hash);
            const cacheKey = `${config.cachePrefix}-${hash}`;
            wasmBytes = await loadFromCache(db, cacheKey, statusCallback);
            if (!wasmBytes) {
                const data = element.innerHTML.trim();
                wasmBytes = await decodeBinary(
                    data, config.shouldDecompress, statusCallback);
                statusCallback?.("Caching module...");
                await saveAssetToCache(db, cacheKey, wasmBytes);
            }
        }

        statusCallback?.(`Initializing ${config.name}...`);
        await config.initFunction(wasmBytes);
        statusCallback?.(`${config.name} ready`);
    }

    // IndexedDB ------------------------------------------------------------- /
    
    const DB_NAME = "HtmlPackerCache";
    const DB_VERSION = 1;
    const ASSET_STORE_NAME = "wasm_cache";
    
    function openDb(statusCallback) {
        return new Promise((resolve, reject) => {
            statusCallback?.("Opening cache database...");
            const request = indexedDB.open(DB_NAME, DB_VERSION);
    
            request.onerror = (e) => {
                console.error("IndexedDB error: ", e.target.error);
                reject("Error opening database...");
            };
    
            request.onupgradeneeded = (e) => {
                const db = e.target.result;
                if (!db.objectStoreNames.contains(ASSET_STORE_NAME)) {
                    statusCallback?.("Creating cache store...");
                    db.createObjectStore(ASSET_STORE_NAME);
                }
            };
    
            request.onsuccess = (e) => {
                statusCallback?.("Cache database ready...");
                resolve(e.target.result);
            };
        });
    }

    async function getAssetFromCache(db, key) {
        return new Promise((resolve, reject) => {
            const transaction = db.transaction([ASSET_STORE_NAME], 'readonly');
            const store = transaction.objectStore(ASSET_STORE_NAME);
            const request = store.get(key);

            request.onerror = (e) => {
                console.error("Error reading from cache: ", e.target.error);
                reject(new Error("Error reading from cache"));

            }
            request.onsuccess = (e) => resolve(e.target.result);
        });
    }

    async function saveAssetToCache(db, key, value) {
        return new Promise((resolve, reject) => {
            const transaction = db.transaction([ASSET_STORE_NAME], 'readwrite');
            const store = transaction.objectStore(ASSET_STORE_NAME);
            const request = store.put(value, key);

            request.onerror = (e) => {
                console.error("Error writing to cache: ", e.target.error);
                reject(new Error("Error writing to cache"));
            }
            request.onsuccess = (e) => {
                console.log(`Asset with key '${key}' cached.`);
                resolve();
            };
        });
    }
})();

/*
* loading
* enhanced loading screen with status indicators
*/
(() => {
    // Export
    window.createLoadingScreen = createLoadingScreen;

    function createLoadingScreen() {
        // Create loading screen container
        const loadingScreen = document.createElement('div');
        loadingScreen.id = 'loading-screen';

        // Create spinner element
        const spinner = document.createElement('div');
        spinner.className = 'spinner';

        // Create loading text
        const loadingText = document.createElement('div');
        loadingText.className = 'loading-text';
        loadingText.textContent = 'Loading WASM application...';

        // Create progress indicator
        const progressBar = document.createElement('div');
        progressBar.className = 'progress-bar';
        const progressFill = document.createElement('div');
        progressFill.className = 'progress-fill';
        progressBar.appendChild(progressFill);

        // Create a style element for our CSS
        const styleElement = document.createElement('style');
        styleElement.textContent = STYLE;
        
        // Append elements to the DOM
        loadingScreen.appendChild(spinner);
        loadingScreen.appendChild(loadingText);
        loadingScreen.appendChild(progressBar);
        document.head.appendChild(styleElement);
        document.body.appendChild(loadingScreen);

        let currentStep = 0;

        // Return an object with methods to control the loading screen
        return {
            // Update the loading text with progress tracking
            updateText: (text, type = 'normal') => {
                loadingText.textContent = text;
                loadingText.className = `loading-text ${type}`;

                // Update progress bar based on known steps
                const stepIndex = progressSteps.findIndex(step => 
                    text
                        .toLowerCase()
                        .includes(step.toLowerCase().split('...')[0])
                );

                if (stepIndex !== -1) {
                    currentStep = Math.max(currentStep, stepIndex);
                    const progress = 
                        ((currentStep + 1) / progressSteps.length) * 100;
                    progressFill.style.width = `${progress}%`;
                }
            },

            // Hide the loading screen
            hide: () => {
                loadingScreen.style.opacity = '0';
                setTimeout(() => {
                    loadingScreen.style.display = 'none';
                    loadingScreen.remove();
                    styleElement.remove();
                }, 10);
                //}, 500);
            },

            // Show the loading screen (in case it was hidden)
            show: () => {
                loadingScreen.style.display = 'flex';
                setTimeout(() => {
                    loadingScreen.style.opacity = '1';
                }, 10);
            }
        };
    }
   
    // Track progress steps
    const progressSteps = [
        'initializing',
        'opening cache database',
        'cache database ready',
        'loading decoder module',
        'checking cache',
        'loaded from cache',
        'decoding module',
        'decompressing module',
        'caching module',
        'initializing decoder module',
        'decoder module ready',
        'loading main application',
        'checking cache',
        'loaded from cache',
        'decoding module',
        'decompressing module',
        'caching module',
        'initializing main application',
        'main application ready',
        'application ready'
    ];

    // muh css
    const STYLE = `
    :root {
        --bg-main:          #ffffff;
        --text-main:        #495057;
        --text-error:       #b22222;
        --text-success:     #4b9c3e;
        --progress-track:   #e9ecef;
        --progress-fill:    #6ca0af;
        --spinner-track:    #e9ecef;
        --spinner-active:   #6ca0af;
    }

    #loading-screen {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background-color: var(--bg-main);
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        z-index: 9999;
        transition: opacity 0.35s;
        font-family: sans-serif;
    }
    
    .spinner {
        width: 5rem;
        height: 5rem;
        border: 0.25rem solid var(--spinner-track);
        border-top: 0.25rem solid var(--spinner-active);
        border-radius: 50%;
        animation: spin 0.8s linear infinite;
        margin-bottom: 1.5rem;
    }
    
    .loading-text {
        font-size: 1.2rem;
        font-weight: 900;
        color: var(--text-main);
        margin-bottom: 1.5rem;
        text-align: center;
        min-height: 1.5rem;
        transition: color 0.3s;
    }
    
    .loading-text.error {
        color: var(--text-error);
    }
    
    .loading-text.success {
        color: var(--text-success);
    }
    
    .progress-bar {
        width: 19rem;
        height: 0.2rem;
        background-color: var(--progress-track);
        border-radius: 0.2rem;
        overflow: hidden;
        opacity: 0.8;
    }
    
    .progress-fill {
        height: 100%;
        background-color: var(--progress-fill);
        width: 0%;
        transition: width 0.3s ease;
        border-radius: 0.2rem;
    }
    
    @keyframes spin {
        0% { transform: rotate(0deg); }
        100% { transform: rotate(360deg); }
    }
    
    @media (max-width: 480px) {
        .progress-bar {
            width: 80%;
            max-width: 19rem;
        }
        .loading-text {
            font-size: 1rem;
            padding: 0 1.25rem;
        }
    }
    `;
})();

