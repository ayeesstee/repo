// Copyright Ian Stewart 2024, All Rights Reserved.

type Architecture = 
    | x86_64 = 0

type Host =
    | Windows = 0
    | Linux = 1

[<Struct>]
type RustBinarySourcePath = NativeSourcePath of string

[<Struct>]
type RustBinaryOutputPath = NativeOutputPath of string

/// <summary>File paths representing a native binary to be packaged.</summary>
[<Struct>]
type RustBinary = { Arch: Architecture;
                      Host: Host;
                      Source: RustBinarySourcePath; 
                      Output: RustBinaryOutputPath }

[<Struct>]
type DotNetAssemblySourcePath = DotNetSourcePath of string

[<Struct>]
type DotNetAssemblyOutputPath = DotNetOutputPath of string

/// <summary>File paths representing a DotNet assembly to be packaged.</summary>
[<Struct>]
type DotNetAssembly = { Arch: Architecture;
                        Host: Host;
                        Source: DotNetAssemblySourcePath; 
                        Output: DotNetAssemblyOutputPath }

/// <summary>The union of all possible file types that can be part of a package.</summary>
type File = 
    | Rust of RustBinary
    | DotNet of DotNetAssembly

/// <summary>Strongly-typed wrapper representing the name of a package.</summary>
[<Struct>]
type PackageName = Name of string

/// <summary>A package defines a set of files that are delivered together.</summary>
/// <param name="name">The name of this package.</param>
type Package(name: PackageName) =
    /// <summary>The files associated with this package.</summary>
    let mutable files: Set<File> = Set.empty

    /// <summary>Adds a file to this package.</summary>
    /// <param name="file">The file to be added to the package.</param>
    /// <returns>This package instance</returns>
    member this.WithFile (file: File) = 
        let files = Set.add file files
        this

    /// <summary>Gets the files associated with this package.</summary>
    /// <returns>The set of files</returns>
    member this.Files = files
