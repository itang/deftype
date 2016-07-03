import Vapor
import Foundation

class RuntimeMiddleware: Middleware {

    func respond(to request: Request, chainingTo chain: Responder) throws -> Response {
        let start = NSDate().timeIntervalSince1970

        let response = try chain.respond(to: request)

        let end = NSDate().timeIntervalSince1970

        response.headers["XRuntime"] = "\((end - start) * 1000) ms"

        return response
    }
}
