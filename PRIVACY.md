# The Shot Card Privacy Policy

**Effective date:** August 18, 2026  
**Project:** The Shot Card  
**Maintainer:** `prototype461y5k` on GitHub  
**Repository:** [github.com/prototype461y5k/the-shot-card](https://github.com/prototype461y5k/the-shot-card)

> **Draft for review.** I am an AI, not a lawyer. This document is a practical transparency statement for the current open-source application, not formal legal advice. A qualified attorney should review it before the project is relied on as a formal privacy notice.

## Summary

The Shot Card is a native macOS application that helps photographers compose Instagram-ready image cards with camera metadata. The current application is designed to process imported photos locally on the user’s Mac. The application does not require an account, does not operate a project-owned backend, and does not intentionally upload photos or EXIF data to the project maintainer.

## Information the application processes

The application may process the following information when the user chooses to use the corresponding features:

| Information | Why it is processed | Where it is processed |
|---|---|---|
| Imported image pixels | Previewing and exporting a composition | Locally on the user’s Mac |
| EXIF metadata | Filling camera and exposure fields after the user selects Fill from EXIF | Locally on the user’s Mac |
| Camera and lens list values | Populating the user-managed editor lists | Locally in the application’s local storage |
| Language preference | Restoring the selected interface language | Locally in the application’s local storage |
| Composition settings | Rendering the selected layout and export | Locally in the application state and native export process |

The application does not intentionally collect names, email addresses, account credentials, precise location, contacts, advertising identifiers, or payment information.

## Local storage

The application stores selected interface preferences and user-managed camera/lens list values in the local browser storage associated with the application window. These values remain on the user’s Mac until the user clears the application’s local data or removes the application data through macOS or another local cleanup process.

## EXIF and location metadata

Some photographs may contain EXIF fields such as camera model, lens, exposure settings, capture date, or location coordinates. The application reads supported metadata only when the user invokes the EXIF fill action. The application does not intentionally transmit that metadata to the project maintainer. Users should review exported images and metadata before sharing them publicly.

## Network activity and third parties

The core editor and native export workflow do not require a project-owned server. The application does not include project-owned analytics, advertising, telemetry, user accounts, or cloud photo storage.

The operating system, the web browser used to download a release, GitHub, package registries used during development, and other infrastructure may process technical information under their own privacy policies. Those services are outside the control of The Shot Card project. The application may also use operating-system facilities such as native dialogs and Finder integration.

## Sharing and disclosure

The project maintainer does not receive imported photos, exported compositions, EXIF fields, or local preference values through the application’s core workflow. A user may independently choose to share an exported image, a source file, an issue report, or diagnostic information through GitHub or another service. That sharing is initiated by the user and is governed by the receiving service’s terms and privacy notice.

## Data retention and deletion

Because the core workflow is local, the project maintainer does not maintain a project-owned copy of imported photos or EXIF data. Users control files saved on their own Mac and should delete local exports, source files, or application data when they no longer want to retain them. Data stored by GitHub, a browser, or another third-party service is subject to that service’s retention and deletion controls.

## Children

The Shot Card is not directed specifically at children and does not knowingly collect personal information from children through a project-owned service.

## Security

Local processing reduces the need to transmit photo data, but no software can guarantee absolute security. Users should download releases from the project’s official GitHub repository, verify the published SHA-256 checksum when practical, keep macOS updated, and review macOS Gatekeeper warnings carefully.

## Changes to this policy

This policy may be updated when the application’s data flows, distribution model, or third-party integrations change. The effective date at the top will be updated when a substantive revision is published.

## Contact

For questions or privacy-related concerns, open a public issue in the [GitHub repository](https://github.com/prototype461y5k/the-shot-card/issues). Do not include private photographs, EXIF data, or other sensitive information in a public issue.
