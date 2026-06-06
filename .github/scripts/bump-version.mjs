import { readFile, writeFile } from "node:fs/promises";

const bump = process.argv[2];
const supportedBumps = new Set(["patch", "minor", "major"]);

if (!supportedBumps.has(bump)) {
  throw new Error(`Expected patch, minor, or major; received: ${bump ?? "<none>"}`);
}

const packagePaths = [
  "package.json",
  "bandersnatch/package.json",
  "bandersnatch/npm/darwin-arm64/package.json",
  "bandersnatch/npm/linux-x64-gnu/package.json",
  "native/package.json",
];
const cargoManifestPaths = [
  "bandersnatch/core/Cargo.toml",
  "bandersnatch/native-binding/Cargo.toml",
  "bandersnatch/wasm-binding/Cargo.toml",
];
const nativePackages = [
  "@typeberry/bandersnatch-native-darwin-arm64",
  "@typeberry/bandersnatch-native-linux-x64-gnu",
];

const rootPackage = JSON.parse(await readFile("package.json", "utf8"));
const currentVersion = rootPackage.version;
const versionParts = currentVersion.match(/^(\d+)\.(\d+)\.(\d+)$/);

if (!versionParts) {
  throw new Error(`Root package version is not a stable semantic version: ${currentVersion}`);
}

let [, major, minor, patch] = versionParts.map(Number);

if (bump === "major") {
  major += 1;
  minor = 0;
  patch = 0;
} else if (bump === "minor") {
  minor += 1;
  patch = 0;
} else {
  patch += 1;
}

const nextVersion = `${major}.${minor}.${patch}`;

function assertCurrentVersion(actual, location) {
  if (actual !== currentVersion) {
    throw new Error(
      `Expected ${location} to use ${currentVersion}, but found ${actual}`,
    );
  }
}

function updateNativeDependencies(pkg, location) {
  for (const dependency of nativePackages) {
    if (pkg.optionalDependencies?.[dependency] !== undefined) {
      assertCurrentVersion(
        pkg.optionalDependencies[dependency],
        `${location} optional dependency ${dependency}`,
      );
      pkg.optionalDependencies[dependency] = nextVersion;
    }
  }
}

for (const packagePath of packagePaths) {
  const pkg = JSON.parse(await readFile(packagePath, "utf8"));
  assertCurrentVersion(pkg.version, packagePath);
  pkg.version = nextVersion;
  updateNativeDependencies(pkg, packagePath);
  await writeFile(packagePath, `${JSON.stringify(pkg, null, 2)}\n`);
}

const packageLockPath = "package-lock.json";
const packageLock = JSON.parse(await readFile(packageLockPath, "utf8"));
const lockPackages = [
  "",
  "bandersnatch",
  "bandersnatch/npm/darwin-arm64",
  "bandersnatch/npm/linux-x64-gnu",
  "native",
];

assertCurrentVersion(packageLock.version, `${packageLockPath} root`);
packageLock.version = nextVersion;

for (const packagePath of lockPackages) {
  const pkg = packageLock.packages?.[packagePath];
  if (!pkg) {
    throw new Error(`Missing ${packageLockPath} entry: ${packagePath || "<root>"}`);
  }

  assertCurrentVersion(
    pkg.version,
    `${packageLockPath} entry ${packagePath || "<root>"}`,
  );
  pkg.version = nextVersion;
  updateNativeDependencies(
    pkg,
    `${packageLockPath} entry ${packagePath || "<root>"}`,
  );
}

await writeFile(packageLockPath, `${JSON.stringify(packageLock, null, 2)}\n`);

for (const manifestPath of cargoManifestPaths) {
  let manifest = await readFile(manifestPath, "utf8");
  const versionLine = `version = "${currentVersion}"`;

  if (!manifest.includes(versionLine)) {
    throw new Error(`Expected ${manifestPath} to contain ${versionLine}`);
  }

  manifest = manifest.replace(versionLine, `version = "${nextVersion}"`);
  await writeFile(manifestPath, manifest);
}

const cargoLockPath = "bandersnatch/Cargo.lock";
let cargoLock = await readFile(cargoLockPath, "utf8");

for (const packageName of [
  "bandersnatch-core",
  "bandersnatch-native",
  "bandersnatch-wasm",
]) {
  const packageVersion = `name = "${packageName}"\nversion = "${currentVersion}"`;

  if (!cargoLock.includes(packageVersion)) {
    throw new Error(`Expected ${cargoLockPath} to contain ${packageVersion}`);
  }

  cargoLock = cargoLock.replace(
    packageVersion,
    `name = "${packageName}"\nversion = "${nextVersion}"`,
  );
}

await writeFile(cargoLockPath, cargoLock);
process.stdout.write(nextVersion);
