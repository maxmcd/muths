const std = @import("std");
const net = std.net;
pub fn main() !void {
    var server = net.StreamServer.init(.{});
    defer server.deinit();
    try server.listen(net.Address.parseIp("127.0.0.1", 8080) catch unreachable);
    std.log.info("listening at {}", .{server.listen_address});
    while (true) {
        const conn = try server.accept();
        conn.stream.close();
        handle_connection(conn);
    }
}

fn handle_connection(conn: std.net.StreamServer.Connection) !void {
    std.log.info("new connection at {}", .{conn.address});
}
