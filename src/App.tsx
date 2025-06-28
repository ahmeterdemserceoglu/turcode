import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import './App.css';
import Layout from './components/Layout';
import Editor from './components/Editor';
import { readTextFile, writeTextFile } from '@tauri-apps/plugin-fs';

interface OpenFile {
  path: string;
  content: string;
  language?: string;
  isDirty: boolean;
}

function App() {
  const [openFiles, setOpenFiles] = useState<OpenFile[]>([]);
  const [activeFileIndex, setActiveFileIndex] = useState<number>(-1);

  useEffect(() => {
    // Listen for file system events
    const unlisten = listen("fs-modified", (event) => {
      // When a file is modified externally, refresh it if it's open
      const filePath = event.payload as string;

      setOpenFiles(prev => {
        const fileIndex = prev.findIndex(file => file.path === filePath);
        if (fileIndex >= 0 && !prev[fileIndex].isDirty) {
          // Reload file content if it's not dirty
          loadFileContent(filePath).then(content => {
            setOpenFiles(prev => {
              const newFiles = [...prev];
              newFiles[fileIndex] = { ...newFiles[fileIndex], content };
              return newFiles;
            });
          });
        }
        return prev;
      });
    });

    return () => {
      unlisten.then(fn => fn());
    };
  }, []);

  // Load file content from disk
  const loadFileContent = async (path: string): Promise<string> => {
    try {
      return await readTextFile(path);
    } catch (err) {
      console.error("Failed to read file:", err);
      return "";
    }
  };

  // Handle file selection from the file explorer
  const handleFileSelect = async (path: string) => {
    // Check if file is already open
    const fileIndex = openFiles.findIndex(file => file.path === path);

    if (fileIndex >= 0) {
      // File is already open, just switch to it
      setActiveFileIndex(fileIndex);
    } else {
      // Load the file content
      const content = await loadFileContent(path);

      // Try to determine language from file extension
      const result = await invoke<{ language_id: string }>('get_syntax_highlighting', {
        filePath: path,
        content: ''
      }).catch(() => ({ language_id: 'plaintext' }));

      // Add to open files
      setOpenFiles(prev => [...prev, {
        path,
        content,
        language: result.language_id,
        isDirty: false
      }]);

      // Set as active file
      setActiveFileIndex(openFiles.length);
    }
  };

  // Handle file content change
  const handleFileChange = (content: string) => {
    if (activeFileIndex >= 0) {
      setOpenFiles(prev => {
        const newFiles = [...prev];
        newFiles[activeFileIndex] = {
          ...newFiles[activeFileIndex],
          content,
          isDirty: true
        };
        return newFiles;
      });
    }
  };

  // Handle file save
  const handleSaveFile = async () => {
    if (activeFileIndex >= 0) {
      const activeFile = openFiles[activeFileIndex];

      try {
        await writeTextFile(activeFile.path, activeFile.content);

        // Update file state to not dirty
        setOpenFiles(prev => {
          const newFiles = [...prev];
          newFiles[activeFileIndex] = {
            ...newFiles[activeFileIndex],
            isDirty: false
          };
          return newFiles;
        });
      } catch (err) {
        console.error("Failed to save file:", err);
      }
    }
  };

  return (
    <Layout onFileSelect={handleFileSelect}>
      {activeFileIndex >= 0 && openFiles[activeFileIndex] ? (
        <Editor
          path={openFiles[activeFileIndex].path}
          content={openFiles[activeFileIndex].content}
          language={openFiles[activeFileIndex].language}
          onChange={handleFileChange}
          onSave={handleSaveFile}
        />
      ) : (
        <div className="h-full flex items-center justify-center text-gray-400">
          <div className="text-center">
            <h2 className="text-2xl mb-4">TurkCode IDE</h2>
            <p>Türk geliştiriciler için tasarlanmış modern entegre geliştirme ortamı</p>
            <p className="text-sm mt-8">Başlamak için sol menüden bir dosya açın veya yeni bir proje oluşturun</p>
          </div>
        </div>
      )}
    </Layout>
  );
}

export default App;
