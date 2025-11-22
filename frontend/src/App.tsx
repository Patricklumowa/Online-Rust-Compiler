import { useState, useRef, useEffect } from "react";
import Editor from "@monaco-editor/react";
import {
  Button,
  Card,
  CardBody,
  CardHeader,
  Divider,
  Navbar,
  NavbarBrand,
  NavbarContent,
  NavbarItem,
  Progress,
  Chip,
} from "@heroui/react";
import { Play, Terminal, Code2, Cpu, Trash2 } from "lucide-react";
import BackgroundAnimation from "./components/BackgroundAnimation";

function App() {
  const [code, setCode] = useState<string>(`use std::io;
use std::io::Write;

fn main() {
    print!("Enter your name: ");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    println!("Hello, {}!", name.trim());
}`);
  const [output, setOutput] = useState<string>("");
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const outputEndRef = useRef<HTMLDivElement>(null);
  const socketRef = useRef<WebSocket | null>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  // Auto-scroll to bottom of output
  useEffect(() => {
    outputEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [output]);

  // Cleanup socket on unmount
  useEffect(() => {
    return () => {
      if (socketRef.current) {
        socketRef.current.close();
      }
    };
  }, []);

  const handleRun = () => {
    if (socketRef.current) {
      socketRef.current.close();
    }

    setIsLoading(true);
    setOutput("");

    const ws = new WebSocket("ws://localhost:3001/ws");
    socketRef.current = ws;

    ws.onopen = () => {
      // Send the code as the first message
      ws.send(JSON.stringify({ code }));
    };

    ws.onmessage = (event) => {
      const data = event.data;
      setOutput((prev) => prev + data);
    };

    ws.onclose = () => {
      setIsLoading(false);
      socketRef.current = null;
    };

    ws.onerror = (error) => {
      console.error("WebSocket error:", error);
      setOutput((prev) => prev + "\nError connecting to server.\n");
      setIsLoading(false);
    };
  };

  const handleInput = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      const value = e.currentTarget.value;
      if (socketRef.current && socketRef.current.readyState === WebSocket.OPEN) {
        socketRef.current.send(value + "\n");
        // Local echo
        setOutput((prev) => prev + value + "\n");
        e.currentTarget.value = "";
      }
    }
  };

  const handleClear = () => {
    setOutput("");
  };

  return (
    <div className="h-screen w-full flex flex-col bg-background text-foreground overflow-hidden dark selection:bg-primary/30 relative">
      <BackgroundAnimation isActive={isLoading} />
      {/* Navbar */}
      <Navbar isBordered maxWidth="full" className="bg-content1/50 backdrop-blur-md border-b border-divider z-10">
        <NavbarBrand>
          <div className="bg-primary/20 p-2 rounded-lg mr-3">
            <Cpu className="text-primary w-6 h-6" />
          </div>
          <div className="flex flex-col">
            <p className="font-bold text-inherit text-lg tracking-tight">Compily</p>
            <p className="text-xs text-default-500">Powered by Axum & Vite</p>
          </div>
        </NavbarBrand>
        <NavbarContent justify="end">
          <NavbarItem>
            <Button 
              color="danger" 
              variant="light" 
              startContent={<Trash2 size={18} />}
              onPress={handleClear}
              isDisabled={!output}
            >
              Clear Output
            </Button>
          </NavbarItem>
          <NavbarItem>
            <Button 
              color="primary" 
              variant="shadow"
              startContent={!isLoading && <Play size={18} fill="currentColor" />}
              onPress={handleRun} 
              isLoading={isLoading}
              className="font-semibold"
            >
              {isLoading ? "Compiling..." : "Run Code"}
            </Button>
          </NavbarItem>
        </NavbarContent>
      </Navbar>

      {/* Main Content */}
      <div className="flex-1 flex flex-row overflow-hidden p-4 gap-4 z-10">
        {/* Editor Pane */}
        <Card className="flex-1 h-full border border-divider shadow-lg bg-content1/50 backdrop-blur-sm">
          <CardHeader className="flex gap-3 px-4 py-3 border-b border-divider">
            <Code2 size={20} className="text-default-500" />
            <div className="flex flex-col">
              <p className="text-md font-semibold">main.rs</p>
            </div>
            <Chip size="sm" variant="flat" color="secondary" className="ml-auto">Rust 1.85</Chip>
          </CardHeader>
          <Divider className="bg-divider" />
          <CardBody className="p-0 overflow-hidden flex-1">
            <div className="h-full w-full">
              <Editor
                height="100%"
                width="100%"
                defaultLanguage="rust"
                theme="vs-dark"
                value={code}
                onChange={(value) => setCode(value || "")}
                options={{
                  minimap: { enabled: false },
                  fontSize: 14,
                  fontFamily: "'JetBrains Mono', 'Fira Code', monospace",
                  scrollBeyondLastLine: false,
                  automaticLayout: true,
                  padding: { top: 16, bottom: 16 },
                  smoothScrolling: true,
                  cursorBlinking: "smooth",
                  cursorSmoothCaretAnimation: "on",
                }}
              />
            </div>
          </CardBody>
        </Card>

        {/* Output Pane */}
        <Card className="flex-1 h-full border border-divider shadow-lg bg-content1/50 backdrop-blur-sm">
          <CardHeader className="flex justify-between items-center px-4 py-3 border-b border-divider">
            <div className="flex gap-3 items-center">
              <Terminal size={20} className="text-success-500" />
              <p className="text-md font-semibold">Terminal Output</p>
            </div>
            {isLoading && (
              <div className="flex items-center gap-2">
                <span className="text-xs text-success-400 animate-pulse">Compiling & Running...</span>
              </div>
            )}
          </CardHeader>
          
          {/* Loading Progress Bar */}
          {isLoading && (
            <Progress
              size="sm"
              isIndeterminate
              aria-label="Loading..."
              className="w-full"
              color="success"
              classNames={{
                track: "drop-shadow-md border border-default",
                indicator: "bg-gradient-to-r from-green-500 to-yellow-500",
              }}
            />
          )}
          
          <CardBody className="p-0 overflow-hidden flex-1 bg-black">
            <div 
              className="w-full h-full overflow-auto p-4 cursor-text"
              onClick={() => inputRef.current?.focus()}
            >
              {output || isLoading ? (
                <div className="font-mono text-sm text-green-400 leading-relaxed m-0 whitespace-pre-wrap break-all">
                  {output}
                  {isLoading && (
                    <div className="flex items-center mt-1">
                      <span className="mr-2 text-gray-500">{">"}</span>
                      <input
                        ref={inputRef}
                        type="text"
                        className="bg-transparent border-none outline-none text-green-400 flex-1 font-mono text-sm"
                        onKeyDown={handleInput}
                        autoComplete="off"
                        autoFocus
                      />
                    </div>
                  )}
                  <div ref={outputEndRef} />
                </div>
              ) : (
                <div className="h-full flex flex-col items-center justify-center text-gray-500 gap-2">
                  <Terminal size={48} className="opacity-20" />
                  <p className="text-sm">Ready to compile...</p>
                </div>
              )}
            </div>
          </CardBody>
        </Card>
      </div>
    </div>
  );
}

export default App;
