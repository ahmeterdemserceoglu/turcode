import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { FolderIcon, FileIcon, ChevronRightIcon, ChevronDownIcon } from './icons';

interface FileInfo {
    name: string;
    path: string;
    is_dir: boolean;
    size: number;
    modified?: string;
}

interface FileExplorerProps {
    onFileSelect: (path: string) => void;
}

const FileExplorer = ({ onFileSelect }: FileExplorerProps) => {
    const [currentPath, setCurrentPath] = useState<string | null>(null);
    const [files, setFiles] = useState<FileInfo[]>([]);
    const [expandedFolders, setExpandedFolders] = useState<Record<string, boolean>>({});
    const [isLoading, setIsLoading] = useState(false);

    // Open a folder
    const handleOpenFolder = async () => {
        try {
            const selected = await open({
                directory: true,
                multiple: false,
                title: 'Select a project folder',
            });

            if (selected && typeof selected === 'string') {
                setCurrentPath(selected);
                loadFiles(selected);
            }
        } catch (err) {
            console.error('Error opening folder:', err);
        }
    };

    // Load files from a specific path
    const loadFiles = async (path: string) => {
        setIsLoading(true);
        try {
            const filesResult = await invoke<FileInfo[]>('list_files', { path });
            setFiles(filesResult);
        } catch (err) {
            console.error('Error listing files:', err);
        } finally {
            setIsLoading(false);
        }
    };

    // Toggle folder expansion
    const toggleFolder = async (folder: FileInfo) => {
        const newExpandedFolders = { ...expandedFolders };
        newExpandedFolders[folder.path] = !expandedFolders[folder.path];
        setExpandedFolders(newExpandedFolders);

        if (newExpandedFolders[folder.path]) {
            loadFiles(folder.path);
        }
    };

    // Handle file click
    const handleFileClick = (file: FileInfo) => {
        if (!file.is_dir) {
            onFileSelect(file.path);
        }
    };

    // Render file or folder item
    const renderItem = (item: FileInfo) => {
        const isExpanded = expandedFolders[item.path];

        return (
            <div key={item.path} className="flex flex-col">
                <div
                    className="flex items-center p-1 hover:bg-background-dark/10 cursor-pointer"
                    onClick={() => item.is_dir ? toggleFolder(item) : handleFileClick(item)}
                >
                    <div className="w-4 h-4 mr-1">
                        {item.is_dir && (isExpanded ? <ChevronDownIcon /> : <ChevronRightIcon />)}
                    </div>
                    <div className="w-5 h-5 mr-2">
                        {item.is_dir ? <FolderIcon /> : <FileIcon />}
                    </div>
                    <span className="text-sm truncate">{item.name}</span>
                </div>

                {/* Render children if folder is expanded */}
                {item.is_dir && isExpanded && (
                    <div className="ml-4">
                        {files
                            .filter(file => file.path.startsWith(item.path) && file.path !== item.path)
                            .map(renderItem)}
                    </div>
                )}
            </div>
        );
    };

    return (
        <div className="h-full bg-background-paper-dark border-r border-gray-700">
            <div className="p-2">
                <button
                    className="w-full p-1 bg-primary text-white text-sm rounded"
                    onClick={handleOpenFolder}
                >
                    Open Folder
                </button>
            </div>

            <div className="p-2">
                {isLoading ? (
                    <div>Loading...</div>
                ) : currentPath ? (
                    <div>
                        {files
                            .filter(file => file.path.includes(currentPath) && file.path !== currentPath)
                            .map(renderItem)}
                    </div>
                ) : (
                    <div className="text-sm text-gray-400">No folder opened</div>
                )}
            </div>
        </div>
    );
};

export default FileExplorer; 