// Copyright Ian Stewart 2024, All Rights Reserved.

#load "fsharp/repo/Packages.fsx"
open Packages

let mercuryGameClient = 
    Package(Name("MercuryGameClient"))
    |> _.WithFile(Rust {Arch = Architecture.x86_64; 
                        Host = Host.Windows; 
                        Source = NativeSourcePath("mercury_game_client.exe"); 
                        Output = NativeOutputPath("mercury_game_client.exe")})
    |> _.WithFile(Rust {Arch = Architecture.x86_64; 
                        Host = Host.Linux; 
                        Source = NativeSourcePath("mercury_game_client"); 
                        Output = NativeOutputPath("mercury_game_client")})

let mercuryGameServer = 
    Package(Name("MercuryGameServer"))
    |> _.WithFile(Rust {Arch = Architecture.x86_64; 
                        Host = Host.Windows; 
                        Source = NativeSourcePath("mercury_game_server.exe"); 
                        Output = NativeOutputPath("mercury_game_server.exe")})
    |> _.WithFile(Rust {Arch = Architecture.x86_64; 
                        Host = Host.Linux; 
                        Source = NativeSourcePath("mercury_game_server"); 
                        Output = NativeOutputPath("mercury_game_server")})