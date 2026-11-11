#![cfg(feature = "typescript")]

use code_gen::typescript::*;
use code_gen::*;

#[test]
fn struct_class() {
    let constructor: Constructor = Constructor::default()
        .with_param(("x", "number"))
        .with_param(("y", "number"))
        .with_semi("this.x = x")
        .with_semi("this.y = y");
    let decode: Function = Function::from(
        Signature::from("decode")
            .with_param(("bytes", "Uint8Array"))
            .with_result("Point"),
    )
    .with_comment(" Decodes the point from the bytes.")
    .with_modifier(Modifier::Static)
    .with_semi("const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength)")
    .with_semi("return new Point(view.getInt32(0, true), view.getInt32(4, true))");
    let equals: Function = Function::from(
        Signature::from("equals")
            .with_param(("other", "Point"))
            .with_result("boolean"),
    )
    .with_comment(" Checks if the point equals the other point.")
    .with_semi("return this.x === other.x && this.y === other.y");
    let encode: Function = Function::from(Signature::from("encode").with_result("Uint8Array"))
        .with_comment(" Encodes the point.")
        .with_semi("const bytes = new Uint8Array(Point.ENCODED_LENGTH)")
        .with_semi("const view = new DataView(bytes.buffer)")
        .with_semi("view.setInt32(0, this.x, true)")
        .with_semi("view.setInt32(4, this.y, true)")
        .with_semi("return bytes");
    let class: Class = Class::from("Point")
        .with_comment(" A point on a grid.")
        .with_modifier(Modifier::Export)
        .with_field(
            Field::from(("ENCODED_LENGTH", "number"))
                .with_comment(" The length of an encoded point in bytes.")
                .with_modifier(Modifier::Static)
                .with_modifier(Modifier::Readonly)
                .with_initializer("8"),
        )
        .with_field(Field::from(("x", "number")).with_comment(" The x coordinate."))
        .with_field(Field::from(("y", "number")).with_comment(" The y coordinate."))
        .with_constructor(constructor)
        .with_function(decode)
        .with_function(equals)
        .with_function(encode);

    assert_eq!(
        class.to_code(),
        r#"/** A point on a grid. */
export class Point {
    /** The length of an encoded point in bytes. */
    static readonly ENCODED_LENGTH: number = 8;

    /** The x coordinate. */
    x: number;

    /** The y coordinate. */
    y: number;

    constructor(x: number, y: number) {
        this.x = x;
        this.y = y;
    }

    /** Decodes the point from the bytes. */
    static decode(bytes: Uint8Array): Point {
        const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
        return new Point(view.getInt32(0, true), view.getInt32(4, true));
    }

    /** Checks if the point equals the other point. */
    equals(other: Point): boolean {
        return this.x === other.x && this.y === other.y;
    }

    /** Encodes the point. */
    encode(): Uint8Array {
        const bytes = new Uint8Array(Point.ENCODED_LENGTH);
        const view = new DataView(bytes.buffer);
        view.setInt32(0, this.x, true);
        view.setInt32(4, this.y, true);
        return bytes;
    }
}
"#
    );
}

#[test]
fn message_class() {
    let fields: SwitchStatement = SwitchStatement::from("reader.readTagNumber()")
        .with_switch_case(SwitchCase::from("1").with_semi("user.name = reader.readString()"))
        .with_switch_case(
            SwitchCase::from("2")
                .with_semi("user.tags = reader.readList(() => reader.readString())"),
        )
        .with_switch_case(SwitchCase::default().with_semi("reader.skip()"));
    let decode: Function = Function::from(
        Signature::from("decode")
            .with_param(("bytes", "Uint8Array"))
            .with_result("User"),
    )
    .with_comment(" Decodes the user from the bytes.")
    .with_modifier(Modifier::Static)
    .with_semi("const reader = new Reader(bytes)")
    .with_semi("const user = new User()")
    .with_statement(WhileLoop::from("reader.hasMore()").with_statement(fields))
    .with_semi("return user");

    let name: IfStatement = IfStatement::from("this.name !== undefined").with_success_statements(
        Block::default()
            .with_semi("writer.writeTagNumber(1)")
            .with_semi("writer.writeString(this.name)"),
    );
    let tags: IfStatement = IfStatement::from("this.tags !== undefined").with_success_statements(
        Block::default()
            .with_semi("writer.writeTagNumber(2)")
            .with_semi("writer.writeLength(this.tags.length)")
            .with_statement(
                ForLoop::from("const tag of this.tags").with_semi("writer.writeString(tag)"),
            ),
    );
    let encode_to: Function = Function::from(
        Signature::from("encodeTo")
            .with_param(("writer", "Writer"))
            .with_result("void"),
    )
    .with_comment(" Encodes the user to the writer.")
    .with_statement(name)
    .with_statement(tags);

    let class: Class = Class::from("User")
        .with_comment(" A user.")
        .with_modifier(Modifier::Export)
        .with_field(
            Field::from(Var::from(("name", "string")).with_optional(true))
                .with_comment(" The name of the user."),
        )
        .with_field(
            Field::from(
                Var::from(("tags", TypeScriptType::from("string").to_array())).with_optional(true),
            )
            .with_comment(" The tags of the user."),
        )
        .with_function(decode)
        .with_function(encode_to);

    assert_eq!(
        class.to_code(),
        r#"/** A user. */
export class User {
    /** The name of the user. */
    name?: string;

    /** The tags of the user. */
    tags?: string[];

    /** Decodes the user from the bytes. */
    static decode(bytes: Uint8Array): User {
        const reader = new Reader(bytes);
        const user = new User();
        while (reader.hasMore()) {
            switch (reader.readTagNumber()) {
                case 1:
                    user.name = reader.readString();
                    break;
                case 2:
                    user.tags = reader.readList(() => reader.readString());
                    break;
                default:
                    reader.skip();
                    break;
            }
        }
        return user;
    }

    /** Encodes the user to the writer. */
    encodeTo(writer: Writer): void {
        if (this.name !== undefined) {
            writer.writeTagNumber(1);
            writer.writeString(this.name);
        }
        if (this.tags !== undefined) {
            writer.writeTagNumber(2);
            writer.writeLength(this.tags.length);
            for (const tag of this.tags) {
                writer.writeString(tag);
            }
        }
    }
}
"#
    );
}

#[test]
fn enum_with_unrecognized_case() {
    let color: Enum = Enum::from("Color")
        .with_comment(" A color.")
        .with_comment("")
        .with_comment(" A tag number without a case is an unrecognized color.")
        .with_modifier(Modifier::Export)
        .with_case(EnumCase::from("Red").with_value("1"))
        .with_case(EnumCase::from("Green").with_value("2"))
        .with_case(EnumCase::from("Blue").with_value("3"));
    let returns = |case: SwitchCase, value: &str| {
        case.with_break(false)
            .with_semi(format!("return {}", value))
    };
    let names: SwitchStatement = SwitchStatement::from("color")
        .with_switch_case(returns(SwitchCase::from("Color.Red"), "\"Red\""))
        .with_switch_case(returns(SwitchCase::from("Color.Green"), "\"Green\""))
        .with_switch_case(returns(SwitchCase::from("Color.Blue"), "\"Blue\""))
        .with_switch_case(returns(SwitchCase::default(), "undefined"));
    let color_name: Function = Function::from(
        Signature::from("colorName")
            .with_param(("color", "Color"))
            .with_result(TypeScriptType::from("string").to_optional()),
    )
    .with_comment(" Gets the name of the color, which an unrecognized color doesn't have.")
    .with_modifier(Modifier::Export)
    .with_statement(names);
    let block: Block = Block::default()
        .with_statement(color)
        .with_empty_line()
        .with_statement(color_name);

    assert_eq!(
        block.to_code(),
        r#"/**
 * A color.
 *
 * A tag number without a case is an unrecognized color.
 */
export enum Color {
    Red = 1,
    Green = 2,
    Blue = 3,
}

/** Gets the name of the color, which an unrecognized color doesn't have. */
export function colorName(color: Color): string | undefined {
    switch (color) {
        case Color.Red:
            return "Red";
        case Color.Green:
            return "Green";
        case Color.Blue:
            return "Blue";
        default:
            return undefined;
    }
}
"#
    );
}

#[test]
fn variant_union() {
    let readonly = |name: &str, typescript_type: &str| {
        Field::from((name, typescript_type)).with_modifier(Modifier::Readonly)
    };
    let circle: Interface = Interface::from("ShapeCircle")
        .with_comment(" The circle case of a shape.")
        .with_modifier(Modifier::Export)
        .with_field(readonly("kind", "\"circle\""))
        .with_field(readonly("value", "Circle"));
    let square: Interface = Interface::from("ShapeSquare")
        .with_comment(" The square case of a shape.")
        .with_modifier(Modifier::Export)
        .with_field(readonly("kind", "\"square\""))
        .with_field(readonly("value", "Square"));
    let unrecognized: Interface = Interface::from("ShapeUnrecognized")
        .with_comment(" A case of a shape with an unrecognized tag number.")
        .with_modifier(Modifier::Export)
        .with_field(readonly("kind", "\"unrecognized\""))
        .with_field(readonly("tagNumber", "number"))
        .with_field(readonly("bytes", "Uint8Array"));
    let shape: TypeDec = TypeDec::from((
        "Shape",
        TypeScriptType::from("ShapeCircle")
            .with_member("ShapeSquare")
            .with_member("ShapeUnrecognized"),
    ))
    .with_comment(" A shape.")
    .with_modifier(Modifier::Export);

    let cases: SwitchStatement = SwitchStatement::from("tagNumber")
        .with_switch_case(
            SwitchCase::from("1")
                .with_braces(true)
                .with_break(false)
                .with_semi("const value = Circle.decode(reader)")
                .with_semi("return { kind: \"circle\", value }"),
        )
        .with_switch_case(
            SwitchCase::from("2")
                .with_braces(true)
                .with_break(false)
                .with_semi("const value = Square.decode(reader)")
                .with_semi("return { kind: \"square\", value }"),
        )
        .with_switch_case(
            SwitchCase::default()
                .with_braces(true)
                .with_break(false)
                .with_semi("const bytes = reader.readBytes()")
                .with_semi("return { kind: \"unrecognized\", tagNumber, bytes }"),
        );
    let decode: Function = Function::from(
        Signature::from("decodeShape")
            .with_param(("reader", "Reader"))
            .with_result("Shape"),
    )
    .with_comment(" Decodes the shape from the reader.")
    .with_modifier(Modifier::Export)
    .with_statement(VarInit::from((
        "tagNumber",
        Literal::from("reader.readTagNumber()"),
    )))
    .with_statement(cases);

    let block: Block = Block::default()
        .with_statement(circle)
        .with_empty_line()
        .with_statement(square)
        .with_empty_line()
        .with_statement(unrecognized)
        .with_empty_line()
        .with_statement(shape)
        .with_empty_line()
        .with_statement(decode);

    assert_eq!(
        block.to_code(),
        r#"/** The circle case of a shape. */
export interface ShapeCircle {
    readonly kind: "circle";
    readonly value: Circle;
}

/** The square case of a shape. */
export interface ShapeSquare {
    readonly kind: "square";
    readonly value: Square;
}

/** A case of a shape with an unrecognized tag number. */
export interface ShapeUnrecognized {
    readonly kind: "unrecognized";
    readonly tagNumber: number;
    readonly bytes: Uint8Array;
}

/** A shape. */
export type Shape = ShapeCircle | ShapeSquare | ShapeUnrecognized;

/** Decodes the shape from the reader. */
export function decodeShape(reader: Reader): Shape {
    const tagNumber = reader.readTagNumber();
    switch (tagNumber) {
        case 1: {
            const value = Circle.decode(reader);
            return { kind: "circle", value };
        }
        case 2: {
            const value = Square.decode(reader);
            return { kind: "square", value };
        }
        default: {
            const bytes = reader.readBytes();
            return { kind: "unrecognized", tagNumber, bytes };
        }
    }
}
"#
    );
}

#[test]
fn view_interface() {
    let view: Interface = Interface::from("PointView")
        .with_comment(" The fields of any form of a point.")
        .with_modifier(Modifier::Export)
        .with_field(
            Field::from(("x", "number"))
                .with_comment(" The x coordinate.")
                .with_modifier(Modifier::Readonly),
        )
        .with_field(
            Field::from(("y", "number"))
                .with_comment(" The y coordinate.")
                .with_modifier(Modifier::Readonly),
        );

    let getter = |name: &str, offset: usize| {
        Function::from(Signature::from(name).with_result("number"))
            .with_modifier(Modifier::Get)
            .with_semi(format!("return this.#view.getInt32({}, true)", offset))
    };
    let point_ref: Class = Class::from("PointRef")
        .with_comment(" A point that is read from its encoded bytes.")
        .with_modifier(Modifier::Export)
        .with_implements("PointView")
        .with_field(Field::from(("#view", "DataView")).with_modifier(Modifier::Readonly))
        .with_constructor(
            Constructor::default()
                .with_param(("bytes", "Uint8Array"))
                .with_semi(
                    "this.#view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength)",
                ),
        )
        .with_function(getter("x", 0))
        .with_function(getter("y", 4));

    let block: Block = Block::default()
        .with_statement(view)
        .with_empty_line()
        .with_statement(point_ref);

    assert_eq!(
        block.to_code(),
        r#"/** The fields of any form of a point. */
export interface PointView {
    /** The x coordinate. */
    readonly x: number;

    /** The y coordinate. */
    readonly y: number;
}

/** A point that is read from its encoded bytes. */
export class PointRef implements PointView {
    readonly #view: DataView;

    constructor(bytes: Uint8Array) {
        this.#view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    }

    get x(): number {
        return this.#view.getInt32(0, true);
    }

    get y(): number {
        return this.#view.getInt32(4, true);
    }
}
"#
    );
}

#[test]
fn service_interface() {
    let call = |name: &str, request: &str, response: &str, comment: &str| {
        SignatureDec::from(
            Signature::from(name)
                .with_param(("request", request))
                .with_result(TypeScriptType::from("Promise").with_generic(response)),
        )
        .with_comment(comment)
    };
    let greeter: Interface = Interface::from("Greeter")
        .with_comment(" Greets people.")
        .with_modifier(Modifier::Export)
        .with_signature_dec(call(
            "greet",
            "GreetRequest",
            "GreetResponse",
            " Greets a person.",
        ))
        .with_signature_dec(call(
            "farewell",
            "FarewellRequest",
            "FarewellResponse",
            " Says goodbye to a person.",
        ));

    let calls: SwitchStatement = SwitchStatement::from("call")
        .with_switch_case(
            SwitchCase::from("\"greet\"")
                .with_break(false)
                .with_semi("return (await service.greet(GreetRequest.decode(body))).encode()"),
        )
        .with_switch_case(
            SwitchCase::from("\"farewell\"")
                .with_break(false)
                .with_semi(
                    "return (await service.farewell(FarewellRequest.decode(body))).encode()",
                ),
        )
        .with_switch_case(
            SwitchCase::default()
                .with_break(false)
                .with_semi("throw new Error(`unknown call: ${call}`)"),
        );
    let route: Function = Function::from(
        Signature::from("routeGreeter")
            .with_param(("service", "Greeter"))
            .with_param(("call", "string"))
            .with_param(("body", "Uint8Array"))
            .with_result(TypeScriptType::from("Promise").with_generic("Uint8Array")),
    )
    .with_comment(" Routes the call to the service.")
    .with_modifier(Modifier::Export)
    .with_modifier(Modifier::Async)
    .with_statement(calls);

    let block: Block = Block::default()
        .with_statement(greeter)
        .with_empty_line()
        .with_statement(route);

    assert_eq!(
        block.to_code(),
        r#"/** Greets people. */
export interface Greeter {
    /** Greets a person. */
    greet(request: GreetRequest): Promise<GreetResponse>;

    /** Says goodbye to a person. */
    farewell(request: FarewellRequest): Promise<FarewellResponse>;
}

/** Routes the call to the service. */
export async function routeGreeter(service: Greeter, call: string, body: Uint8Array): Promise<Uint8Array> {
    switch (call) {
        case "greet":
            return (await service.greet(GreetRequest.decode(body))).encode();
        case "farewell":
            return (await service.farewell(FarewellRequest.decode(body))).encode();
        default:
            throw new Error(`unknown call: ${call}`);
    }
}
"#
    );
}

#[test]
fn javascript_class() {
    let point: Class = Class::from("Point")
        .with_comment(" A point on a grid.")
        .with_modifier(Modifier::Export)
        .with_field("x")
        .with_field("y")
        .with_constructor(
            Constructor::default()
                .with_param("x")
                .with_param("y")
                .with_semi("this.x = x")
                .with_semi("this.y = y"),
        )
        .with_function(
            Function::from(Signature::from("equals").with_param("other"))
                .with_comment(" Checks if the point equals the other point.")
                .with_semi("return this.x === other.x && this.y === other.y"),
        );
    let origin: VarInit = VarInit::from(("ORIGIN", Literal::from("new Point(0, 0)")))
        .with_comment(" The point where the axes meet.")
        .with_modifier(Modifier::Export);
    let block: Block = Block::default()
        .with_statement(point)
        .with_empty_line()
        .with_statement(origin);

    assert_eq!(
        block.to_code(),
        r#"/** A point on a grid. */
export class Point {
    x;
    y;

    constructor(x, y) {
        this.x = x;
        this.y = y;
    }

    /** Checks if the point equals the other point. */
    equals(other) {
        return this.x === other.x && this.y === other.y;
    }
}

/** The point where the axes meet. */
export const ORIGIN = new Point(0, 0);
"#
    );
}
