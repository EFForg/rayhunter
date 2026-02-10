# Analysis

Start analysis for QMDL file with name `{name}`.

**URL**: `/api/analysis/{name}`

**Method**: `POST`

**Data Constraints**: none

## Success Response

**Code**: 202

**Content**:

```
ok
```

## Error Response

### Code 500

**Condition**: Unable to queue file.

**Content Example**:

```
failed to queue new analysis files
```
