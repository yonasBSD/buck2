// @generated
// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: javacd.proto

// Protobuf Java Version: 3.25.6
package com.facebook.buck.cd.model.java;

/**
 * Protobuf enum {@code javacd.api.v1.BuildMode}
 */
@javax.annotation.Generated(value="protoc", comments="annotations:BuildMode.java.pb.meta")
public enum BuildMode
    implements com.google.protobuf.ProtocolMessageEnum {
  /**
   * <code>BUILD_MODE_UNSPECIFIED = 0;</code>
   */
  BUILD_MODE_UNSPECIFIED(0),
  /**
   * <pre>
   * Represents a build command that generates full library
   * </pre>
   *
   * <code>LIBRARY = 1;</code>
   */
  LIBRARY(1),
  /**
   * <pre>
   * Represents a build command that generates abi
   * </pre>
   *
   * <code>ABI = 2;</code>
   */
  ABI(2),
  UNRECOGNIZED(-1),
  ;

  /**
   * <code>BUILD_MODE_UNSPECIFIED = 0;</code>
   */
  public static final int BUILD_MODE_UNSPECIFIED_VALUE = 0;
  /**
   * <pre>
   * Represents a build command that generates full library
   * </pre>
   *
   * <code>LIBRARY = 1;</code>
   */
  public static final int LIBRARY_VALUE = 1;
  /**
   * <pre>
   * Represents a build command that generates abi
   * </pre>
   *
   * <code>ABI = 2;</code>
   */
  public static final int ABI_VALUE = 2;


  public final int getNumber() {
    if (this == UNRECOGNIZED) {
      throw new java.lang.IllegalArgumentException(
          "Can't get the number of an unknown enum value.");
    }
    return value;
  }

  /**
   * @param value The numeric wire value of the corresponding enum entry.
   * @return The enum associated with the given numeric wire value.
   * @deprecated Use {@link #forNumber(int)} instead.
   */
  @java.lang.Deprecated
  public static BuildMode valueOf(int value) {
    return forNumber(value);
  }

  /**
   * @param value The numeric wire value of the corresponding enum entry.
   * @return The enum associated with the given numeric wire value.
   */
  public static BuildMode forNumber(int value) {
    switch (value) {
      case 0: return BUILD_MODE_UNSPECIFIED;
      case 1: return LIBRARY;
      case 2: return ABI;
      default: return null;
    }
  }

  public static com.google.protobuf.Internal.EnumLiteMap<BuildMode>
      internalGetValueMap() {
    return internalValueMap;
  }
  private static final com.google.protobuf.Internal.EnumLiteMap<
      BuildMode> internalValueMap =
        new com.google.protobuf.Internal.EnumLiteMap<BuildMode>() {
          public BuildMode findValueByNumber(int number) {
            return BuildMode.forNumber(number);
          }
        };

  public final com.google.protobuf.Descriptors.EnumValueDescriptor
      getValueDescriptor() {
    if (this == UNRECOGNIZED) {
      throw new java.lang.IllegalStateException(
          "Can't get the descriptor of an unrecognized enum value.");
    }
    return getDescriptor().getValues().get(ordinal());
  }
  public final com.google.protobuf.Descriptors.EnumDescriptor
      getDescriptorForType() {
    return getDescriptor();
  }
  public static final com.google.protobuf.Descriptors.EnumDescriptor
      getDescriptor() {
    return com.facebook.buck.cd.model.java.JavaCDProto.getDescriptor().getEnumTypes().get(0);
  }

  private static final BuildMode[] VALUES = values();

  public static BuildMode valueOf(
      com.google.protobuf.Descriptors.EnumValueDescriptor desc) {
    if (desc.getType() != getDescriptor()) {
      throw new java.lang.IllegalArgumentException(
        "EnumValueDescriptor is not for this type.");
    }
    if (desc.getIndex() == -1) {
      return UNRECOGNIZED;
    }
    return VALUES[desc.getIndex()];
  }

  private final int value;

  private BuildMode(int value) {
    this.value = value;
  }

  // @@protoc_insertion_point(enum_scope:javacd.api.v1.BuildMode)
}

