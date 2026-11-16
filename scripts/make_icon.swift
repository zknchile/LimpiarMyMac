import CoreGraphics
import Foundation
import ImageIO
import UniformTypeIdentifiers

// Cristal plano. Fondo de tinta y tres facetas de la paleta de día lluvioso.
// No es vidrio: no hay brillo, sombra ni reflejo.

let ink = rgba(0x0F, 0x17, 0x2A)
let cloud = rgba(0xE2, 0xE8, 0xF0)
let band = rgba(0xD2, 0xDA, 0xE5)
let hair = rgba(0x94, 0xA3, 0xB8)

let root = CommandLine.arguments.count > 1
    ? URL(fileURLWithPath: CommandLine.arguments[1], isDirectory: true)
    : URL(fileURLWithPath: FileManager.default.currentDirectoryPath, isDirectory: true)

let sizes = [16, 32, 64, 128, 256, 512, 1024]
for size in sizes {
    let png = draw(size: size)
    let name = size == 1024 ? "icon.png" : "icon-\(size).png"
    try png.write(to: root.appendingPathComponent(name))
}

func draw(size: Int) -> Data {
    let width = size
    let space = CGColorSpace(name: CGColorSpace.sRGB)!
    guard let ctx = CGContext(
        data: nil,
        width: width,
        height: width,
        bitsPerComponent: 8,
        bytesPerRow: 0,
        space: space,
        bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue
    ) else {
        fputs("no se pudo crear el lienzo\n", stderr)
        exit(1)
    }
    ctx.translateBy(x: 0, y: CGFloat(width))
    ctx.scaleBy(x: 1, y: -1)
    ctx.setFillColor(ink)
    ctx.fill(CGRect(x: 0, y: 0, width: width, height: width))

    let s = CGFloat(width) / 1024
    func p(_ x: CGFloat, _ y: CGFloat) -> CGPoint {
        CGPoint(x: x * s, y: y * s)
    }
    let top = p(512, 188)
    let upperRight = p(756, 360)
    let lowerRight = p(692, 648)
    let bottom = p(512, 836)
    let lowerLeft = p(332, 648)
    let upperLeft = p(268, 360)
    let center = p(512, 492)

    // El contorno va primero. Si cada faceta se pinta sola, el borde
    // suavizado deja ver la tinta entre ellas.
    fill(ctx, [top, upperRight, lowerRight, bottom, lowerLeft, upperLeft], band)
    fill(ctx, [top, upperRight, center, upperLeft], cloud)
    fill(ctx, [upperRight, lowerRight, bottom, center], hair)

    guard let image = ctx.makeImage() else {
        fputs("no se pudo cerrar el icono\n", stderr)
        exit(1)
    }
    let data = NSMutableData()
    guard let dest = CGImageDestinationCreateWithData(data, UTType.png.identifier as CFString, 1, nil) else {
        fputs("no se pudo abrir el PNG\n", stderr)
        exit(1)
    }
    CGImageDestinationAddImage(dest, image, nil)
    guard CGImageDestinationFinalize(dest) else {
        fputs("no se pudo escribir el PNG\n", stderr)
        exit(1)
    }
    return data as Data
}

func fill(_ ctx: CGContext, _ points: [CGPoint], _ color: CGColor) {
    ctx.beginPath()
    ctx.move(to: points[0])
    for point in points.dropFirst() {
        ctx.addLine(to: point)
    }
    ctx.closePath()
    ctx.setFillColor(color)
    ctx.fillPath()
}

func rgba(_ r: CGFloat, _ g: CGFloat, _ b: CGFloat) -> CGColor {
    CGColor(srgbRed: r / 255, green: g / 255, blue: b / 255, alpha: 1)
}
