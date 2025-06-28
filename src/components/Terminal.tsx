import { useState, useRef, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Command } from '@tauri-apps/plugin-shell';

interface TerminalProps {
    cwd?: string;
}

interface TerminalLine {
    content: string;
    type: 'input' | 'output' | 'error';
}

const Terminal = ({ cwd = '.' }: TerminalProps) => {
    const [lines, setLines] = useState<TerminalLine[]>([]);
    const [currentInput, setCurrentInput] = useState<string>('');
    const [commandHistory, setCommandHistory] = useState<string[]>([]);
    const [historyPosition, setHistoryPosition] = useState<number>(-1);
    const [isRunning, setIsRunning] = useState<boolean>(false);

    const terminalRef = useRef<HTMLDivElement>(null);
    const inputRef = useRef<HTMLInputElement>(null);

    // Auto scroll to bottom when new lines are added
    useEffect(() => {
        if (terminalRef.current) {
            terminalRef.current.scrollTop = terminalRef.current.scrollHeight;
        }
    }, [lines]);

    // Focus the input when terminal is clicked
    const focusInput = () => {
        if (inputRef.current) {
            inputRef.current.focus();
        }
    };

    // Execute a shell command
    const executeCommand = async (command: string) => {
        if (!command.trim()) return;

        // Add command to history
        setLines(prev => [...prev, { content: `$ ${command}`, type: 'input' }]);
        setCommandHistory(prev => [command, ...prev.slice(0, 19)]);
        setHistoryPosition(-1);
        setCurrentInput('');

        try {
            setIsRunning(true);

            // Create new command
            const cmd = Command.sidecar('terminal', [command]);

            // Set current working directory
            cmd.setCurrentDirectory(cwd);

            // Listen for command output
            cmd.on('close', data => {
                const exitCode = data.code;
                if (exitCode !== 0) {
                    setLines(prev => [...prev, {
                        content: `Command exited with code ${exitCode}`,
                        type: 'error'
                    }]);
                }
                setIsRunning(false);
            });

            cmd.on('error', error => {
                setLines(prev => [...prev, {
                    content: `Error: ${error}`,
                    type: 'error'
                }]);
                setIsRunning(false);
            });

            cmd.stdout.on('data', line => {
                setLines(prev => [...prev, { content: line, type: 'output' }]);
            });

            cmd.stderr.on('data', line => {
                setLines(prev => [...prev, { content: line, type: 'error' }]);
            });

            // Execute the command
            await cmd.spawn();
        } catch (error) {
            setLines(prev => [...prev, {
                content: `Failed to execute command: ${error}`,
                type: 'error'
            }]);
            setIsRunning(false);
        }
    };

    // Handle input submission
    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        if (!isRunning) {
            executeCommand(currentInput);
        }
    };

    // Handle keyboard navigation through history
    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (isRunning) return;

        // Navigate through command history
        if (e.key === 'ArrowUp') {
            e.preventDefault();
            if (historyPosition < commandHistory.length - 1) {
                const newPosition = historyPosition + 1;
                setHistoryPosition(newPosition);
                setCurrentInput(commandHistory[newPosition]);
            }
        } else if (e.key === 'ArrowDown') {
            e.preventDefault();
            if (historyPosition > 0) {
                const newPosition = historyPosition - 1;
                setHistoryPosition(newPosition);
                setCurrentInput(commandHistory[newPosition]);
            } else if (historyPosition === 0) {
                setHistoryPosition(-1);
                setCurrentInput('');
            }
        }
    };

    return (
        <div className="h-full flex flex-col bg-[#1E1E1E] text-white font-mono text-sm overflow-hidden">
            <div className="flex-1 overflow-auto p-2" ref={terminalRef} onClick={focusInput}>
                {lines.map((line, i) => (
                    <div
                        key={i}
                        className={`whitespace-pre-wrap mb-1 ${line.type === 'error' ? 'text-red-400' :
                                line.type === 'input' ? 'text-yellow-400' : 'text-white'
                            }`}
                    >
                        {line.content}
                    </div>
                ))}

                {/* Current command line */}
                <form onSubmit={handleSubmit} className="flex">
                    <span className="text-yellow-400 mr-1">$</span>
                    <input
                        ref={inputRef}
                        type="text"
                        value={currentInput}
                        onChange={(e) => setCurrentInput(e.target.value)}
                        onKeyDown={handleKeyDown}
                        disabled={isRunning}
                        className="flex-1 bg-transparent outline-none"
                        autoFocus
                    />
                </form>
            </div>
        </div>
    );
};

export default Terminal; 