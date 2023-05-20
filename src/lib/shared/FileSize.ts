
export default class FileSize {

    bits: number = 0;
    bytes: number = 0;

    static bytesInKiloByte = 1000;
    static bytesInMegaByte = 1000000;
    static bytesInGigaByte = 1000000000;
    static bytesInTeraByte = 1000000000000;
    static bytesInPetaByte = 1000000000000000;

    constructor(bits: number) {
        this.bits = bits;
        this.bytes = bits / 8;
    }

    static fromBits(bits: number) {
        return new FileSize(bits);
    }

    static fromBytes(bytes: number) {
        return new FileSize(Math.ceil(bytes * 8));
    }

    static fromKiloBytes(value: number) {
        return this.fromBytes(value * this.bytesInKiloByte);
    }

    static fromMegaBytes(value: number) {
        return this.fromBytes(value * this.bytesInMegaByte);
    }

    static fromGigaBytes(value: number) {
        return this.fromBytes(value * this.bytesInGigaByte);
    }

    static fromTeraBytes(value: number) {
        return this.fromBytes(value * this.bytesInTeraByte);
    }

    static fromPetaBytes(value: number) {
        return this.fromBytes(value * this.bytesInPetaByte);
    }

    static fromKiloBits(value: number) {
        return this.fromBits(value * this.bytesInKiloByte);
    }

    static fromMegaBits(value: number) {
        return this.fromBits(value * this.bytesInMegaByte);
    }

    static fromGigaBits(value: number) {
        return this.fromBits(value * this.bytesInGigaByte);
    }

    static fromTeraBits(value: number) {
        return this.fromBits(value * this.bytesInTeraByte);
    }

    static fromPetaBits(value: number) {
        return this.fromBits(value * this.bytesInPetaByte);
    }

    toKiloBytes() : number {
        return this.bytes / FileSize.bytesInKiloByte;
    }
    toMegaBytes() : number {
        return this.bytes / FileSize.bytesInMegaByte;
    }
    toGigaBytes() : number {
        return this.bytes / FileSize.bytesInGigaByte;
    }
    toTeraBytes() : number {
        return this.bytes / FileSize.bytesInTeraByte;
    }
    toPetaBytes() : number {
        return this.bytes / FileSize.bytesInPetaByte;
    }


    toKiloBits() : number {
        return this.bits / FileSize.bytesInKiloByte;
    }
    toMegaBits() : number {
        return this.bits / FileSize.bytesInMegaByte;
    }
    toGigaBits() : number {
        return this.bits / FileSize.bytesInGigaByte;
    }
    toTeraBits() : number {
        return this.bits / FileSize.bytesInTeraByte;
    }
    toPetaBits() : number {
        return this.bits / FileSize.bytesInPetaByte;
    }
    
}