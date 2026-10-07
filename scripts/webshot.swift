// Renders a URL in an offscreen WKWebView and writes a PNG snapshot.
// usage: swift scripts/webshot.swift <url> <out.png> [width] [height] [delay-seconds] [js-to-run-before-snapshot]
import AppKit
import WebKit

let args = CommandLine.arguments
guard args.count >= 3 else {
    FileHandle.standardError.write("usage: webshot <url> <out.png> [width] [height] [delay] [js]\n".data(using: .utf8)!)
    exit(2)
}
let url = URL(string: args[1])!
let out = URL(fileURLWithPath: args[2])
let width = args.count > 3 ? Double(args[3])! : 1440
let height = args.count > 4 ? Double(args[4])! : 900
let delay = args.count > 5 ? Double(args[5])! : 2.0
let script = args.count > 6 ? args[6] : nil

class Delegate: NSObject, WKNavigationDelegate {
    func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
        DispatchQueue.main.asyncAfter(deadline: .now() + delay) {
            let snap = {
                DispatchQueue.main.asyncAfter(deadline: .now() + 0.8) {
                    let config = WKSnapshotConfiguration()
                    webView.takeSnapshot(with: config) { image, error in
                        guard let image = image, let tiff = image.tiffRepresentation,
                              let rep = NSBitmapImageRep(data: tiff),
                              let png = rep.representation(using: .png, properties: [:]) else {
                            print("snapshot failed: \(String(describing: error))")
                            exit(1)
                        }
                        try! png.write(to: out)
                        exit(0)
                    }
                }
            }
            if let js = script {
                webView.evaluateJavaScript(js) { _, err in
                    if let err = err { print("js error: \(err)") }
                    snap()
                }
            } else {
                snap()
            }
        }
    }
}

let app = NSApplication.shared
app.setActivationPolicy(.prohibited)
let window = NSWindow(contentRect: NSRect(x: -10000, y: -10000, width: width, height: height),
                      styleMask: [.borderless], backing: .buffered, defer: false)
let webView = WKWebView(frame: NSRect(x: 0, y: 0, width: width, height: height))
let delegate = Delegate()
webView.navigationDelegate = delegate
window.contentView = webView
window.orderBack(nil)
webView.load(URLRequest(url: url))
DispatchQueue.main.asyncAfter(deadline: .now() + 30) { print("timeout"); exit(1) }
app.run()
