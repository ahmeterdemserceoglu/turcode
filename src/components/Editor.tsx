import { useEffect, useRef, useState } from 'react';
import { Editor as MonacoEditor } from '@monaco-editor/react';
import { editor } from 'monaco-editor/esm/vs/editor/editor.api';
import { invoke } from '@tauri-apps/api/core';

interface EditorProps {
    path?: string;
    content?: string;
    language?: string;
    onChange?: (value: string) => void;
    onSave?: () => void;
}

const Editor = ({ path, content = '', language, onChange, onSave }: EditorProps) => {
    const [editorValue, setEditorValue] = useState(content);
    const editorRef = useRef<editor.IStandaloneCodeEditor | null>(null);
    const monacoRef = useRef<any>(null);

    // Handle editor mount
    const handleEditorDidMount = (editor: editor.IStandaloneCodeEditor, monaco: any) => {
        editorRef.current = editor;
        monacoRef.current = monaco;

        // Add keyboard shortcut for saving
        editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyS, () => {
            if (onSave) {
                onSave();
            }
        });

        // Set initial focus
        editor.focus();
    };

    // Get syntax highlighting for the file
    useEffect(() => {
        if (!path) return;

        const getSyntaxHighlighting = async () => {
            try {
                const result = await invoke<{ language_id: string, tokens: any[] }>('get_syntax_highlighting', {
                    filePath: path,
                    content: editorValue
                });

                // If language wasn't explicitly provided, use the detected one
                if (!language && result.language_id && monacoRef.current) {
                    // Set the language
                    const model = editorRef.current?.getModel();
                    if (model) {
                        monacoRef.current.editor.setModelLanguage(model, result.language_id);
                    }
                }
            } catch (err) {
                console.error('Failed to get syntax highlighting:', err);
            }
        };

        getSyntaxHighlighting();
    }, [path, editorValue, language]);

    // Handle content changes
    const handleEditorChange = (value: string | undefined) => {
        const newValue = value || '';
        setEditorValue(newValue);
        if (onChange) {
            onChange(newValue);
        }
    };

    return (
        <div className="h-full w-full">
            <MonacoEditor
                height="100%"
                width="100%"
                language={language}
                theme="vs-dark"
                value={editorValue}
                onChange={handleEditorChange}
                onMount={handleEditorDidMount}
                options={{
                    minimap: { enabled: true },
                    fontSize: 14,
                    fontFamily: 'JetBrains Mono, monospace',
                    wordWrap: 'on',
                    automaticLayout: true,
                    tabSize: 2,
                    scrollBeyondLastLine: false,
                    rulers: [80, 100],
                    bracketPairColorization: { enabled: true },
                }}
            />
        </div>
    );
};

export default Editor; 