import java.io.IOException;
import java.text.SimpleDateFormat;
import java.util.Properties;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import javax.mail.Folder;
import javax.mail.Message;
import javax.mail.MessagingException;
import javax.mail.NoSuchProviderException;
import javax.mail.Session;
import javax.mail.Store;
import javax.mail.internet.MimeMultipart;
import javax.mail.Part;
import org.jsoup.Jsoup;
import java.util.Date;
import javax.mail.*;

public class Mail {

    // Define the fixed polling parameters
    private static final long MAX_WAIT_SECONDS = 10 * 60; // 10 minutes
    private static final long POLLING_INTERVAL_SECONDS = 15; // 15 seconds

    /**
     * Attempts to retrieve a six-digit number from the last email received AFTER the
     * function started checking. It polls the inbox with a fixed max wait of 5 minutes
     * and a polling interval of 15 seconds. It uses receivedDate or falls back to sentDate
     * for comparison.
     *
     * @param host                 The email server host (e.g., "pop.gmail.com").
     * @param storeType            The store type (e.g., "pop3s").
     * @param user                 The email username.
     * @param password             The email password.
     * @return The found six-digit number, or null if not found within the timeout.
     */
    public static String getSixDigitNumberFromLastEmail(
            String host, String storeType, String user, String password) {

        String sixDigitNumber = null;
        Store store = null;
        Folder emailFolder = null;

        Date startTime = new Date(); // Capture the "actual time" when polling begins
        long endTimeMillis = System.currentTimeMillis() + (MAX_WAIT_SECONDS * 1000);
        long twoSecondsAgoMillis = startTime.getTime() - (2 * 1000);
        //Date pastTime = new Date(twoSecondsAgoMillis);
        startTime.setTime(twoSecondsAgoMillis);
        
        System.out.println("Starting to poll for a new email received AFTER: " + startTime);
        SimpleDateFormat timeFormatter24Hour = new SimpleDateFormat("HH:mm:ss");

        while (System.currentTimeMillis() < endTimeMillis && sixDigitNumber == null) {
            try {
                Properties properties = new Properties();
                properties.put("mail.pop3.host", host);
                properties.put("mail.pop3.port", "995");
                properties.put("mail.pop3.starttls.enable", "true");
                properties.put("mail.pop3.ssl.enable", "true");

                Session emailSession = Session.getDefaultInstance(properties);
                store = emailSession.getStore("pop3s");
                store.connect(host, user, password);

                emailFolder = store.getFolder("INBOX");
                emailFolder.open(Folder.READ_ONLY);

                Message[] messages = emailFolder.getMessages();
                System.out.println("\nPolling... Total messages in inbox: " + messages.length);

                if (messages.length > 0) {
                    Message lastMessage = messages[messages.length - 1];

                    // --- Start of new date logic ---
                    Date messageDate = lastMessage.getReceivedDate();
                    String dateType = "Received";

                    // Fallback to Sent Date if Received Date is null
                    if (messageDate == null) {
                        messageDate = lastMessage.getSentDate();
                        dateType = "Sent (fallback)";
                    }
                    // --- End of new date logic ---

                    String formattedMessageTime = (messageDate != null) ? timeFormatter24Hour.format(messageDate) : "N/A";
                    System.out.println("  Last Message " + dateType + " Date/Time: " + formattedMessageTime);

                    // Check if a valid date was obtained AND it is strictly AFTER our startTime
                    if (messageDate != null && messageDate.after(startTime)) {
                        System.out.println("  *** Found a new email (" + dateType + " date after " + startTime + "). Processing... ***");
                        String emailText = getTextFromMessage(lastMessage);
                        System.out.println("  Extracted Email Text (first 200 chars):\n" + (emailText != null && emailText.length() > 200 ? emailText.substring(0, 200) + "..." : emailText));

                        sixDigitNumber = findSixDigitNumber(emailText, "\\b\\d{6}\\b");

                        if (sixDigitNumber != null) {
                            System.out.println("  $$$ Successfully found 6-digit number: " + sixDigitNumber + " $$$");
                            break;
                        } else {
                            System.out.println("  New email found, but no 6-digit number extracted from it. Continuing to poll...");
                        }
                    } else {
                        System.out.println("  Last email is not new enough (or no valid date found). Waiting for a new email...");
                    }
                } else {
                    System.out.println("  No emails found in the inbox. Waiting for a new email...");
                }

            } catch (NoSuchProviderException e) {
                System.err.println("Error: No such mail provider. " + e.getMessage());
                sixDigitNumber = null;
                break;
            } catch (MessagingException e) {
                System.err.println("Error during messaging operation: " + e.getMessage());
                sixDigitNumber = null;
            } catch (IOException e) {
                System.err.println("Error reading email content: " + e.getMessage());
                sixDigitNumber = null;
            } finally {
                if (emailFolder != null && emailFolder.isOpen()) {
                    try {
                        emailFolder.close(false);
                    } catch (MessagingException e) {
                        System.err.println("Error closing email folder in polling loop: " + e.getMessage());
                    }
                }
                if (store != null && store.isConnected()) {
                    try {
                        store.close();
                    } catch (MessagingException e) {
                        System.err.println("Error closing mail store in polling loop: " + e.getMessage());
                    }
                }
            }

            if (sixDigitNumber == null && System.currentTimeMillis() < endTimeMillis) {
                try {
                    System.out.println("  Next check in " + POLLING_INTERVAL_SECONDS + " seconds...");
                    Thread.sleep(POLLING_INTERVAL_SECONDS * 1000);
                } catch (InterruptedException ie) {
                    Thread.currentThread().interrupt();
                    System.err.println("Polling was interrupted.");
                    sixDigitNumber = null;
                    break;
                }
            }
        }

        if (sixDigitNumber == null) {
            System.out.println("\nTimed out after " + MAX_WAIT_SECONDS + " seconds without finding a new email with a 6-digit number.");
        }
        return sixDigitNumber;
    }

    // --- Helper methods (getTextFromMessage, getTextFromMimeMultipart, findSixDigitNumber, extractPlainTextFromHtml) ---

    private static String getTextFromMessage(Message message) throws MessagingException, IOException {
        String result = "";
        String contentType = message.getContentType();
        System.out.println("  Message Content Type: " + contentType);

        if (contentType != null) {
            if (contentType.toLowerCase().contains("text/plain")) {
                result = message.getContent().toString();
            } else if (contentType.toLowerCase().contains("text/html")) {
                result = extractPlainTextFromHtml(message.getContent().toString());
            } else if (contentType.toLowerCase().contains("multipart/")) {
                MimeMultipart mimeMultipart = (MimeMultipart) message.getContent();
                result = getTextFromMimeMultipart(mimeMultipart);
            }
        }
        return result;
    }

    private static String getTextFromMimeMultipart(MimeMultipart mimeMultipart) throws MessagingException, IOException {
        StringBuilder result = new StringBuilder();
        int count = mimeMultipart.getCount();
        String plainTextPart = null;
        String htmlTextPart = null;

        for (int i = 0; i < count; i++) {
            Part bodyPart = mimeMultipart.getBodyPart(i);
            String contentType = bodyPart.getContentType();
            System.out.println("    Part " + i + " Content Type: " + contentType);

            try {
                Object content = bodyPart.getContent();
                if (contentType != null) {
                    if (contentType.toLowerCase().contains("text/plain")) {
                        plainTextPart = content.toString();
                    } else if (contentType.toLowerCase().contains("text/html")) {
                        htmlTextPart = extractPlainTextFromHtml(content.toString());
                    } else if (contentType.toLowerCase().contains("multipart/")) {
                        result.append(getTextFromMimeMultipart((MimeMultipart) content));
                    }
                }
            } catch (IOException | MessagingException e) {
                System.err.println("    Error getting content of part " + i + ": " + e.getMessage());
            }
        }

        if (plainTextPart != null && !plainTextPart.trim().isEmpty()) {
            result.append(plainTextPart);
        } else if (htmlTextPart != null && !htmlTextPart.trim().isEmpty()) {
            result.append(htmlTextPart);
        }

        return result.toString().trim();
    }

    private static String findSixDigitNumber(String text, String regex) {
        if (text == null) {
            return null;
        }
        Pattern pattern = Pattern.compile(regex);
        Matcher matcher = pattern.matcher(text);
        if (matcher.find()) {
            return matcher.group();
        } else {
            return null;
        }
    }

    private static String extractPlainTextFromHtml(String html) {
        if (html == null) {
            return null;
        }
        try {
            org.jsoup.nodes.Document doc = Jsoup.parse(html);
            return doc.text().replaceAll("\\s+", " ").trim();
        } catch (Exception e) {
            System.err.println("Error extracting plain text from HTML: " + e.getMessage());
            return null;
        }
    }
    
    public static String getLastEmailText(
            String host, String storeType, String user, String password) {

        Store store = null;
        Folder emailFolder = null;
        String emailText = null;

        Date startTime = new Date(System.currentTimeMillis() - 8000);
        long endTimeMillis = System.currentTimeMillis() + (MAX_WAIT_SECONDS * 1000);

        while (System.currentTimeMillis() < endTimeMillis && emailText == null) {
            try {
                Properties properties = new Properties();
                properties.put("mail.pop3.host", host);
                properties.put("mail.pop3.port", "995");
                properties.put("mail.pop3.starttls.enable", "true");
                properties.put("mail.pop3.ssl.enable", "true");

                Session emailSession = Session.getDefaultInstance(properties);
                store = emailSession.getStore(storeType);
                store.connect(host, user, password);

                emailFolder = store.getFolder("INBOX");
                emailFolder.open(Folder.READ_ONLY);

                Message[] messages = emailFolder.getMessages();

                if (messages.length > 0) {
                    Message lastMessage = messages[messages.length - 1];

                    Date messageDate = lastMessage.getReceivedDate();
                    if (messageDate == null) {
                        messageDate = lastMessage.getSentDate();
                    }

                    if (messageDate != null && messageDate.after(startTime)) {
                        emailText = extractText(lastMessage);
                        if (emailText != null && !emailText.trim().isEmpty()) {
                            return emailText.trim();
                        }
                    }
                }

            } catch (Exception e) {
                e.printStackTrace();
            } finally {
                try {
                    if (emailFolder != null && emailFolder.isOpen()) {
                        emailFolder.close(false);
                    }
                } catch (MessagingException ignored) {}

                try {
                    if (store != null && store.isConnected()) {
                        store.close();
                    }
                } catch (MessagingException ignored) {}
            }

            if (emailText == null && System.currentTimeMillis() < endTimeMillis) {
                try {
                    Thread.sleep(POLLING_INTERVAL_SECONDS * 1000);
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                    return null;
                }
            }
        }

        return null;
    }

    private static String extractText(Part part) throws MessagingException, IOException {
        if (part.isMimeType("text/plain")) {
            return (String) part.getContent();
        }

        if (part.isMimeType("text/html")) {
            String html = (String) part.getContent();
            return Jsoup.parse(html).text();
        }

        if (part.isMimeType("multipart/alternative")) {
            Multipart multipart = (Multipart) part.getContent();
            String text = null;
            String html = null;

            for (int i = 0; i < multipart.getCount(); i++) {
                BodyPart bodyPart = multipart.getBodyPart(i);

                if (bodyPart.isMimeType("text/plain")) {
                    text = extractText(bodyPart);
                } else if (bodyPart.isMimeType("text/html")) {
                    html = extractText(bodyPart);
                } else {
                    String nested = extractText(bodyPart);
                    if (nested != null && !nested.isBlank()) {
                        return nested;
                    }
                }
            }

            return text != null ? text : html;
        }

        if (part.isMimeType("multipart/*")) {
            Multipart multipart = (Multipart) part.getContent();
            StringBuilder result = new StringBuilder();

            for (int i = 0; i < multipart.getCount(); i++) {
                BodyPart bodyPart = multipart.getBodyPart(i);

                String disposition = bodyPart.getDisposition();
                if (Part.ATTACHMENT.equalsIgnoreCase(disposition)) {
                    continue;
                }

                String text = extractText(bodyPart);
                if (text != null && !text.isBlank()) {
                    result.append(text).append("\n");
                }
            }

            return result.toString().trim();
        }

        return null;
    }

}