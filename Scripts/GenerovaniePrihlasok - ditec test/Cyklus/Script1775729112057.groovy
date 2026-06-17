import com.kms.katalon.core.model.FailureHandling
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

for (int i = 0; i < 3; i++) {
    WebUI.callTestCase(findTestCase("Test Cases/GenerovaniePrihlasok - ditec test/Prihláška na MŠ"), [:], FailureHandling.CONTINUE_ON_FAILURE)
}