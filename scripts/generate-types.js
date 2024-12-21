// scripts/generate-types.js
const { glob } = require('glob');
const { exec } = require('child_process');
const { promisify } = require('util');
const path = require('path');
const fs = require('fs/promises');

const execAsync = promisify(exec);

async function cleanDirectory(dir) {
    try {
        await fs.rm(dir, { recursive: true, force: true });
        await fs.mkdir(dir, { recursive: true });
        console.log(`Cleaned directory: ${dir}`);
    } catch (error) {
        console.error(`Error cleaning directory ${dir}:`, error);
        throw error;
    }
}

async function cleanupRustGeneratedFiles(generatedDir, keepDir = 'models') {
    const files = await fs.readdir(generatedDir);
    for (const file of files) {
        const filePath = path.join(generatedDir, file);
        const stats = await fs.stat(filePath);
        if (stats.isDirectory() && file === 'src') {
            // Move content from src/models to current directory
            await fs.cp(path.join(filePath, keepDir), generatedDir, { recursive: true });
            await fs.rm(filePath, { recursive: true });
        } else if (file !== keepDir) {
            await fs.rm(filePath, { recursive: true });
        }
    }
}

async function createModFile(dir, subModules) {
    const modContent = subModules
        .map(name => `pub mod ${name};`)
        .join('\n');
    await fs.writeFile(path.join(dir, 'mod.rs'), modContent);
}

async function generateTypes() {
    try {
        // Clean generated directories
        await cleanDirectory('src/generated');
        await cleanDirectory('frontend/src/generated');
        
        // Find all yaml files in openapi directory recursively
        const schemaFiles = await glob('openapi/**/*.yaml');
        
        // Keep track of modules at each level
        const moduleTree = new Map();

        for (const schemaFile of schemaFiles) {
            const relativePath = path.relative('openapi', schemaFile);
            const basePathWithoutExt = path.join(path.dirname(relativePath), path.basename(relativePath, '.yaml'));
            
            console.log(`Generating types for ${relativePath}...`);

            // Generate minimal Rust types
            const rustDir = path.join('src/generated', basePathWithoutExt);
            await fs.mkdir(rustDir, { recursive: true });
            
            await execAsync(`openapi-generator-cli generate \
                -i ${schemaFile} \
                -g rust \
                -o ${rustDir} \
                --additional-properties=packageName=${path.basename(basePathWithoutExt).replace(/-/g, '_')}_types,supportAsync=false,enumUnknownDefaultCase=false \
                --global-property=models,supportingFiles=model_mod.rs \
                --skip-validate-spec`);

            // Clean up everything except the models
            await cleanupRustGeneratedFiles(rustDir);

            // Track modules for mod.rs generation
            const pathParts = basePathWithoutExt.split(path.sep);
            let currentPath = 'src/generated';
            for (let i = 0; i < pathParts.length; i++) {
                if (!moduleTree.has(currentPath)) {
                    moduleTree.set(currentPath, new Set());
                }
                moduleTree.get(currentPath).add(pathParts[i]);
                currentPath = path.join(currentPath, pathParts[i]);
            }

            console.log(`✓ Generated Rust types for ${relativePath}`);

            // Generate TypeScript types using openapi-typescript-codegen
            const tsDir = path.join('frontend/src/generated');
            await fs.mkdir(tsDir, { recursive: true });
            
            await execAsync(`npx openapi-typescript-codegen \
                --input ${schemaFile} \
                --output ${tsDir} \
                --exportCore=false \
                --exportServices=false \
                `);
            
            console.log(`✓ Generated TypeScript types for ${relativePath}`);
        }

        // Create mod.rs files for Rust
        for (const [dir, modules] of moduleTree.entries()) {
            await createModFile(dir, Array.from(modules));
        }

    } catch (error) {
        console.error('Error generating types:', error);
        process.exit(1);
    }
}

generateTypes();