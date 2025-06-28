import { ReactNode, useState } from 'react';
import FileExplorer from './FileExplorer';
import { TerminalIcon, CogIcon, BranchIcon } from './icons';

interface LayoutProps {
    children: ReactNode;
    onFileSelect: (path: string) => void;
}

const Layout = ({ children, onFileSelect }: LayoutProps) => {
    const [sidebarWidth, setSidebarWidth] = useState(250);
    const [isDragging, setIsDragging] = useState(false);

    // Sidebar resize handler
    const handleMouseDown = (e: React.MouseEvent) => {
        e.preventDefault();
        setIsDragging(true);

        const handleMouseMove = (e: MouseEvent) => {
            const newWidth = e.clientX;
            if (newWidth > 100 && newWidth < 500) {
                setSidebarWidth(newWidth);
            }
        };

        const handleMouseUp = () => {
            document.removeEventListener('mousemove', handleMouseMove);
            document.removeEventListener('mouseup', handleMouseUp);
            setIsDragging(false);
        };

        document.addEventListener('mousemove', handleMouseMove);
        document.addEventListener('mouseup', handleMouseUp);
    };

    return (
        <div className="flex flex-col h-screen bg-background-dark text-white">
            {/* Top Bar */}
            <header className="h-12 bg-background-paper-dark border-b border-gray-700 flex items-center px-4">
                <h1 className="text-lg font-medium">TurkCode IDE</h1>
                <div className="flex-1" />
                <button className="p-2 rounded hover:bg-background-dark/20">
                    <CogIcon className="w-5 h-5" />
                </button>
            </header>

            {/* Main Content */}
            <div className="flex-1 flex overflow-hidden">
                {/* Sidebar */}
                <div
                    className="bg-background-paper-dark border-r border-gray-700"
                    style={{ width: sidebarWidth, minWidth: sidebarWidth, maxWidth: sidebarWidth }}
                >
                    <div className="h-full flex flex-col">
                        <div className="h-10 bg-background-dark/30 border-b border-gray-700 flex items-center px-3">
                            <span className="text-sm font-medium">Explorer</span>
                        </div>
                        <div className="flex-1 overflow-auto">
                            <FileExplorer onFileSelect={onFileSelect} />
                        </div>
                    </div>
                </div>

                {/* Sidebar resize handle */}
                <div
                    className={`w-1 cursor-col-resize relative z-10 ${isDragging ? 'bg-primary' : 'hover:bg-primary'}`}
                    onMouseDown={handleMouseDown}
                />

                {/* Main Editor Area */}
                <div className="flex-1 flex flex-col overflow-hidden">
                    {children}
                </div>

                {/* Right Sidebar (Git, etc.) */}
                <div className="w-10 border-l border-gray-700 bg-background-paper-dark flex flex-col items-center py-3">
                    <button className="p-2 mb-2 rounded hover:bg-background-dark/20">
                        <BranchIcon className="w-5 h-5" />
                    </button>
                    <button className="p-2 rounded hover:bg-background-dark/20">
                        <TerminalIcon className="w-5 h-5" />
                    </button>
                </div>
            </div>

            {/* Status Bar */}
            <footer className="h-6 bg-primary-dark text-xs flex items-center px-3">
                <span>Ready</span>
                <div className="flex-1" />
                <span>Ln 1, Col 1</span>
                <span className="ml-4">UTF-8</span>
                <span className="ml-4">TypeScript</span>
            </footer>
        </div>
    );
};

export default Layout; 