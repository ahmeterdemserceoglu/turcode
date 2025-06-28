import { useState } from 'react';
import { XMarkIcon } from './icons';
import { basename } from '@tauri-apps/api/path';

interface Tab {
    id: string;
    title: string;
    path: string;
    isDirty: boolean;
}

interface TabManagerProps {
    tabs: Tab[];
    activeTabId: string | null;
    onTabSelect: (tabId: string) => void;
    onTabClose: (tabId: string) => void;
}

const TabManager = ({ tabs, activeTabId, onTabSelect, onTabClose }: TabManagerProps) => {
    const [hoveredTabId, setHoveredTabId] = useState<string | null>(null);

    // Get just the filename from a path
    const getFileName = (filePath: string): string => {
        // Simple basename implementation if Tauri API isn't available
        const parts = filePath.split(/[\/\\]/); // Split by both types of slashes
        return parts[parts.length - 1] || filePath;
    };

    // Handle tab click
    const handleTabClick = (tabId: string) => {
        onTabSelect(tabId);
    };

    // Handle close click
    const handleCloseClick = (e: React.MouseEvent, tabId: string) => {
        e.stopPropagation(); // Prevent triggering tab selection
        onTabClose(tabId);
    };

    return (
        <div className="flex h-10 bg-background-paper-dark border-b border-gray-700 overflow-x-auto">
            {tabs.map((tab) => (
                <div
                    key={tab.id}
                    className={`flex items-center px-3 py-1 border-r border-gray-700 min-w-0 cursor-pointer select-none ${activeTabId === tab.id
                        ? 'bg-background-dark border-b-2 border-b-primary'
                        : 'hover:bg-background-dark/20'
                        }`}
                    onClick={() => handleTabClick(tab.id)}
                    onMouseEnter={() => setHoveredTabId(tab.id)}
                    onMouseLeave={() => setHoveredTabId(null)}
                >
                    <div className="flex items-center max-w-xs">
                        <span className="truncate mr-2">{getFileName(tab.path)}</span>
                        {tab.isDirty && <span className="text-yellow-400 mr-2">•</span>}

                        {/* Only show close button on hover or active tab */}
                        {(hoveredTabId === tab.id || activeTabId === tab.id) && (
                            <button
                                className="w-4 h-4 flex items-center justify-center rounded-sm hover:bg-gray-700"
                                onClick={(e) => handleCloseClick(e, tab.id)}
                                aria-label="Close tab"
                            >
                                <XMarkIcon className="w-3 h-3" />
                            </button>
                        )}
                    </div>
                </div>
            ))}
        </div>
    );
};

export default TabManager; 